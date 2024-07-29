use std::collections::HashMap;

use crate::book::Book;
use crate::poem::Poem;
use crate::tmp_poem::TmpPoem;

pub struct BookBuilder {
    tmp_poem: Option<TmpPoem>,
    tmp_poems: HashMap<u32, TmpPoem>,
    book: Book,
}

impl BookBuilder {
    pub fn new(nn: u32) -> Self {
        Self {
            tmp_poem: None,
            tmp_poems: Default::default(),
            book: Book::new(nn),
        }
    }

    /// Обработка строчки с номером (закрытие текущего, открытие нового с новым номером)
    pub fn proc_number(&mut self, new_nn: u32) {
        let tmp_poem = self.tmp_poem.take();
        match tmp_poem {
            // Закрываем временный объект и открываем новый с новым номером
            Some(tmp_poem) => {
                if self.book.poems.contains_key(&tmp_poem.nn) { panic!("The poem '{}' was already added!", tmp_poem.nn) }
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
                self.tmp_poem = Some(TmpPoem { nn: new_nn, lines: vec![] })
            }
            // В самом начале нечего закрывать открываем новый
            None => {
                self.tmp_poem = Some(TmpPoem { nn: new_nn, lines: vec![] })
            }
        }
    }

    /// Обработка строки стихотворения  
    pub fn proc_line(&mut self, line: String) {
        match &mut self.tmp_poem {
            Some(tmp_poem) => {
                tmp_poem.add(line);
            }
            None => {
                panic!("Can not add line '{}'. Because there was no Number!", line)
            }
        }
    }

    /// Завершение обработки книги. Финализация модели книги.
    pub fn build(mut self) -> Book {
        let tmp_poem = self.tmp_poem.take();
        match tmp_poem {
            // 
            Some(tmp_poem) => {
                if self.book.poems.contains_key(&tmp_poem.nn) { panic!("The poem '{}' was already added!", tmp_poem.nn) }
                let en_poem = self.tmp_poems.remove(&tmp_poem.nn);
                match en_poem {
                    Some(en) => {
                        let p = Poem::new(tmp_poem.nn, en.lines, tmp_poem.lines);
                        self.book.add(p)
                    }
                    None => {
                        panic!("No translation for '{}' poem", tmp_poem.nn)
                    }
                }
            }
            None => {
                panic!("No poems in the book '{}'", self.book.nn);
            }
        }
        return self.book;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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

        let book = builder.build();

        println!("{:#?}", book);
    }
}