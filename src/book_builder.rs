use std::collections::HashMap;
use std::ops::Deref;
use crate::{error::*, utils};
use crate::report::BookReport;
use crate::{Result, Error};
use crate::book::Book;
use crate::poem::Poem;
use crate::tmp_poem::TmpPoem;

pub struct BookBuilder {
    tmp_poem: Option<TmpPoem>,
    tmp_poems: HashMap<u32, TmpPoem>,
    book: Book,
    report: BookReport
}

impl BookBuilder {
    pub fn new(nn: u32) -> Self {
        Self {
            tmp_poem: None,
            tmp_poems: Default::default(),
            book: Book::new(nn),
            report: BookReport::new(nn),
        }
    }

    pub fn add_error(&mut self, error: &Error){
        self.report.add(error);
    }

    pub fn check_if_error<F>(&mut self, f: F) -> Result<()>
    where 
        F: Fn()->Result<()>,
    {
        if let Err(e)=f() {
            self.report.add(&e);
        }
        Ok(())
    }

    pub fn parse_poem_num(&mut self, str: &str) -> u32 {
        let res = utils::parse_poem_num_impl(str);
        res.unwrap_or_else(|e| {self.report.add(&e); 0})
    }

    /// Обработка строчки с номером (закрытие текущего, открытие нового с новым номером)
    pub fn proc_number(&mut self, new_nn: u32) {
        let tmp_poem = self.tmp_poem.take();
        match tmp_poem {
            // Закрываем временный объект и открываем новый с новым номером
            Some(tmp_poem) => {
                if self.book.poems.contains_key(&tmp_poem.nn) {
                    // Ошибка, две поэмы с одним номером
                    self.report.add(&Error::DuplicatePoem{number: tmp_poem.nn});  
                }
                let en_poem = self.tmp_poems.remove(&tmp_poem.nn);
                match en_poem {
                    // Завершен русский перевод, добавляем полностью готовое стихотворенье 
                    Some(en) => {
                        let p = Poem::new(tmp_poem.nn, en.lines, tmp_poem.lines);
                        self.book.add(p)
                    }
                    // Завершаем английскую часть стихотворенья
                    None => {
                        self.tmp_poems.insert(tmp_poem.nn, tmp_poem);
                    }
                }
                // открываем новый с новым номером
                self.tmp_poem = Some(TmpPoem { nn: new_nn, lines: vec![] })
            }
            // В самом начале нечего закрывать открываем новый
            None => {
                self.tmp_poem = Some(TmpPoem { nn: new_nn, lines: vec![] })
            }
        }
        // Ok(())
    }

    /// Обработка строки стихотворения  
    pub fn proc_line(&mut self, line: String) {
        let poem = self.tmp_poem.as_mut();
        match poem {
            Some( p) => {
                p.add_line(line);
            },                
            None => self.report.add(&Error::CanNotAddLine_PoemHasNoNumber{line: line.clone()})
        }
    }

    /// Завершение обработки книги. Финализация модели книги.
    pub fn build(mut self) -> (Option<Book>, BookReport) {
        let tmp_poem = self.tmp_poem.take();
        match tmp_poem {
            // 
            Some(tmp_poem) => {
                if self.book.poems.contains_key(&tmp_poem.nn) {
                    self.report.add(&Error::DuplicatePoem{number: tmp_poem.nn});

                    return (Some(self.book), self.report)
                    // (None, self.report)
                }
                let en_poem = self.tmp_poems.remove(&tmp_poem.nn);
                match en_poem {
                    Some(en) => {
                        let p = Poem::new(tmp_poem.nn, en.lines, tmp_poem.lines);
                        self.book.add(p);

                        return (Some(self.book), self.report)
                    }
                    None => {
                        self.report.add(&Error::NoTranslationForPoem { number: self.book.nn });

                        (Some(self.book), self.report)
                        // (None, self.report)
                    }
                }
            }
            None => {
                self.report.add(&Error::NoPoemsInTheBook { number: self.book.nn });

                (Some(self.book), self.report)
                // (None, self.report)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() ->Result<()>{
        let mut builder = BookBuilder::new(7);
        builder.proc_number(1);
        builder.proc_line(String::from("Qwerty 1 1"));
        builder.proc_line(String::from("Qwerty 1 2"));
        builder.proc_line(String::from("Qwerty 1 3"));
        builder.proc_number(2);
        builder.proc_line(String::from("Qwerty 2 1"));
        builder.proc_line(String::from("Qwerty 2 2"));
        builder.proc_line(String::from("Qwerty 2 3"));
        builder.proc_number(1);
        builder.proc_line(String::from("Йцукен 1 1"));
        builder.proc_line(String::from("Йцукен 1 2"));
        builder.proc_line(String::from("Йцукен 1 3"));
        builder.proc_number(2);
        builder.proc_line(String::from("Йцукен 2 1"));
        builder.proc_line(String::from("Йцукен 2 2"));
        builder.proc_line(String::from("Йцукен 2 3"));

        let (book,err_report) = builder.build();

        println!(">>>> Book: {:#?}", book);
        println!(">>>> Error Report: {:#?}", err_report);
        Ok(())
    }
}