use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use report::BookReport;
use scraper::{Html, Selector};
use tera::Context;
use threadpool::ThreadPool;
use tracing::info;

pub use config::*;
pub use error::*;

use crate::book::Book;
use crate::book_builder::BookBuilder;
use crate::template::TEMPLATES;
use crate::utils::*;

pub mod poem;
pub mod book;
pub mod tmp_poem;
mod book_builder;
mod template;
mod utils;
mod config;
mod error;
pub mod report;

fn main()->Result<()> {
    init_logger();

    let pool = ThreadPool::new(100);
    // let mut file_count = 0;

    prepare_res_dir(CONFIG.res_dir.as_str())?;

    for entry in fs::read_dir(CONFIG.src_dir.as_str())? {
        let path = entry?.path();
        if path.is_file() {
            // file_count += 1;

            {
                pool.execute(move || {
                    process_file(path, CONFIG.res_dir.as_str()).unwrap();
                });
            }
        }
    }

    pool.join();
    //assert_eq!(rx.iter().take(file_count).fold(0, |a, b| a + b), file_count);
    Ok(())
}

fn process_file(src_file_path: PathBuf, res_dir_name: &str)->Result<()> {
    let str = fs::read_to_string(&src_file_path)?;
    let src_file_name = path_2_str(&src_file_path)?;
    let book_num = parse_book_num(src_file_name)?;

    let (book, report) = parse_book(book_num, str.as_str())?;

    // Generate and write Book
    if let Some(book) = book {
        let new_book_text = generate_book(&book)?;
        info!("{}", join_file_path(res_dir_name, src_file_name).to_str().unwrap());
        
        let res = write_book(
            join_file_path(res_dir_name, src_file_name),
            new_book_text.as_str(),
        )?;
        
        info!("Write Book result:{:?}", res);
    }

    // Generate and write report
    let report_text = generate_report(report)?;
    let res = write_report(report_text)?;
    info!("Write report result: {:?}", res);
    Ok(())
}

fn generate_report(report: BookReport) -> Result<String> {
    todo!()
}

fn write_report(report_text: String) -> Result<()> {
    todo!()
}

fn write_book(path: PathBuf, book_text: &str) -> Result<()> {
    let file = fs::OpenOptions::new().create(true).write(true).open(path)?;
    let mut file = BufWriter::new(file);
    file.write_all(book_text.as_bytes()).unwrap();
    file.flush()?;
    Ok(())
}

fn generate_book(book: &Book) -> Result<String> {
    let mut context = Context::new();
    context.insert("book", book);
    context.insert("books", &book.get_ordered_poems());
    // let res = TEMPLATES.render(CONFIG.poem_template.as_str(), &context);
    TEMPLATES.render(CONFIG.poem_template.as_str(), &context).map_err(Error::from)
    // res.unwrap()
}

fn prepare_res_dir(dir_name: &str) -> Result<()> {
    fs::remove_dir_all(dir_name).unwrap();   // Если не удалось удалить - нестрашно
    fs::create_dir_all(dir_name)?;  // Не удалось создать - ошибка
    Ok(())
}

fn parse_book(book_num: u32, html_text: &str) -> Result<(Option<Book>, BookReport)> {
    let mut builder = BookBuilder::new(book_num);
    let document = Html::parse_document(html_text);
    let selector = Selector::parse("body div")
        .map_err(|e|->Error{
            let error = Error::ParseSelectorErrorKind(format!("{:?}",e));
            builder.add_error(&error);
            error
        })?;
    let select = document.select(&selector);
    for q in select.filter(|x| { x.children().count() > 100 }).into_iter() {
        for p in q.child_elements() {
            // let cls = p.attr("class").ok_or(Error::Html{html:p.html()})?;
            let cls = p.attr("class").ok_or_else(|| Error::Html{html:p.html()})?;
            // let cls = match p.attr("class") {
            //     Some(c) => c,
            //     None => { panic!("ОШИБКА!!! {}", p.html()) }
            // };
            match cls {
                "_7_number" | "_7_number-long" => {
                    let num = builder.parse_poem_num(&p.inner_html());
                    builder.proc_number(num);
                    // println!("{}", num)
                }
                "_7_poem" => {
                    let line = p.inner_html();
                    let line = utils::re_remove_tags(&line);
                    builder.proc_line(line);
                    // println!("{}", p.inner_html())
                }
                &_ => {}
            }
            // println!("{}", p.html())
        }
    }
    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_file_name() -> Result<()> {
        let n = 3;
        let file_name = format!("Vol. {:02}.html", n);
        let src_file_path = Path::new(CONFIG.src_dir.as_str()).join(file_name);

        let src_file_name = src_file_path.file_name().map(|s|s.to_str()).flatten()
        .ok_or_else(|| 
            Error::PathError{
                path: src_file_path.as_os_str().to_string_lossy().into()
            });
        info!("===> {src_file_name:?}");
        Ok(())
    }
    #[test]
    fn process_single_file() ->Result<()>{
        let n = 3;
        let file_name = format!("Vol. {:02}.html", n);
        let src = Path::new(CONFIG.src_dir.as_str()).join(file_name);
        let res = process_file(src, CONFIG.res_dir.as_str());
        info!("===> {res:?}");
        Ok(())
    }

    #[test]
    fn test_book_num() -> Result<()> {
        assert_eq!(7, parse_book_num("Vol. 07.html")?);
        Ok(())
    }
}
