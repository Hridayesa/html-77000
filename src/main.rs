use std::fs;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::{PathBuf};
use std::string::ToString;

use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use tera::Context;
use threadpool::ThreadPool;
use tracing::info;

use crate::book::Book;
use crate::book_builder::BookBuilder;
use crate::config::CONFIG;
use crate::template::TEMPLATES;
use crate::utils::{init_logger, join_file_path};

pub mod poem;
pub mod book;
pub mod tmp_poem;
mod book_builder;
mod template;
mod utils;
mod config;


// pub static SRC_DIR_NAME: &str = r"c:\Work\ckg\77 000\77 000";
// pub static RES_DIR_NAME: &str = r"c:\Work\ckg\77 000\77 000_res";


lazy_static! {
    static ref RE_DD: Regex = Regex::new(r"\d\d").unwrap();
    static ref RE_SPEC:Regex = Regex::new(r"&.*?;").unwrap();
    static ref RE_NON_DIGIT:Regex = Regex::new(r"\D+").unwrap();
    static ref RE_TAGS:Regex = Regex::new(r"<.*?>").unwrap();
}

// pub struct Config{
//     pub src_dir_name: String,
//     pub res_dir_name: String,
// }

fn main() {
    init_logger();

    let pool = ThreadPool::new(100);
    // let mut file_count = 0;

    prepare_res_dir(CONFIG.res_dir.as_str());

    for entry in fs::read_dir(CONFIG.src_dir.as_str()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            // file_count += 1;

            {
                pool.execute(move || {
                    process_file(path, CONFIG.res_dir.as_str());
                });
            }
        }
    }

    pool.join();
    //assert_eq!(rx.iter().take(file_count).fold(0, |a, b| a + b), file_count);
}

fn process_file(src_file_path: PathBuf, res_dir_name: &str) {
    let str = fs::read_to_string(src_file_path.to_str().unwrap()).unwrap();
    let src_file_name = src_file_path.file_name().unwrap().to_str().unwrap();
    let book_num = parse_book_num(src_file_name);

    let book = parse_book(book_num, &str);

    // Generate and write Book
    let new_book_text = generate_book(&book);
    info!("{}", join_file_path(res_dir_name, src_file_name).to_str().unwrap());
    let res = write_book(
        join_file_path(res_dir_name, src_file_name),
        new_book_text.as_str(),
    );
    info!("Write Book result:{:?}", res);
    
    // Generate and write report
    let report_text = generate_report();
    let res = write_report();
    info!("Write report result: {:?}", res);
}

fn generate_report() -> String {
    todo!()
}

fn write_book(path: PathBuf, book_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::OpenOptions::new().create(true).write(true).open(path)?;
    let mut file = BufWriter::new(file);
    file.write_all(book_text.as_bytes()).unwrap();
    file.flush()?;
    Ok(())
}

fn generate_book(book: &Book) -> String {
    let mut context = Context::new();
    context.insert("book", book);
    context.insert("books", &book.get_ordered_poems());
    let res = TEMPLATES.render(CONFIG.poem_template.as_str(), &context);
    res.unwrap()
}

fn prepare_res_dir(dir_name: &str) {
    fs::remove_dir_all(dir_name).unwrap();
    fs::create_dir_all(dir_name).unwrap();
}

fn parse_book(book_num: u32, html_text: &str) -> Book {
    let mut builder = BookBuilder::new(book_num);
    let document = Html::parse_document(html_text);
    let selector = Selector::parse("body div").unwrap();
    let select = document.select(&selector);
    for q in select.filter(|x| { x.children().count() > 100 }).into_iter() {
        for p in q.child_elements() {
            let cls = match p.attr("class") {
                Some(c) => c,
                None => { panic!("ОШИБКА!!! {}", p.html()) }
            };
            match cls {
                "_7_number" | "_7_number-long" => {
                    let num = parse_poem_num(&p.inner_html());
                    builder.proc_number(num);
                    // println!("{}", num)
                }
                "_7_poem" => {
                    let line = p.inner_html();
                    let line = re_remove_tags(&line);
                    builder.proc_line(line);
                    // println!("{}", p.inner_html())
                }
                &_ => {}
            }
            // println!("{}", p.html())
        }
    }
    builder.build()
}

fn re_remove_tags(line: &str) -> String {
    RE_TAGS.replace_all(line, "").deref().to_string()
}

/// "Vol.07.html" -> 7
fn parse_book_num(name: &str) -> u32 {
    match RE_DD.find(name) {
        Some(m) => { m.as_str().parse::<u32>().unwrap() }
        None => { panic!("Filename '{}' is unexpected", name) }
    }
}

/// "01&#160;000." -> 1000
/// "03,456." -> 3456
fn parse_poem_num(str: &str) -> u32 {
    let r1 = RE_SPEC.replace_all(str, "");
    let binding = RE_NON_DIGIT.replace_all(r1.deref(), "");
    let x = binding.deref();
    // let len = str.len();
    // let x = format!("{}{}",str[0..2].to_string(), str[len - 4..len - 1].to_string());
    x.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn process_single_file() {
        let n = 3;
        let file_name = format!("Vol. {:02}.html", n);
        let src = Path::new(CONFIG.src_dir.as_str()).join(file_name);
        process_file(src, CONFIG.res_dir.as_str());
    }

    #[test]
    fn test_book_num() {
        assert_eq!(7, parse_book_num("Vol.07.html"))
    }

    #[test]
    fn test_poem_num() {
        assert_eq!(3456, parse_poem_num("03,456."));
        assert_eq!(1000, parse_poem_num("01&#160;000."));
    }
}
