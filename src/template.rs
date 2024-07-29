// extern crate tera;
// #[macro_use]
// extern crate lazy_static;

//#[macro_use]
extern crate lazy_static;//#[macro_use]
extern crate tera;

use lazy_static::lazy_static;
use tera::Tera;
use crate::config::CONFIG;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        
        let tera = match Tera::new(CONFIG.template_pattern.as_str()) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec![".html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

#[cfg(test)]
mod tests {
    use tera::Context;

    use crate::book::tests::get_test_book;

    use super::*;

    #[test]
    fn qqq() {
        let book = get_test_book();
        //let l: Vec<&str> = TEMPLATES.get_template_names().collect();
        println!("qqq{:?}", TEMPLATES.get_template_names().collect::<Vec<&str>>());
        let mut context = Context::new();
        context.insert("book", &book);
        context.insert("books", &book.get_ordered_poems());
        // let res = TEMPLATES.render("poems_77000.html", &context);
        let res = TEMPLATES.render(CONFIG.poem_template.as_str(), &context);
        println!("{}", res.unwrap());
    }
}