use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::poem::Poem;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub nn: u32,
    pub poems: HashMap<u32, Poem>,
}

impl Book {
    pub fn new(nn: u32) -> Self {
        Self {
            nn,
            poems: Default::default(),
        }
    }

    pub fn add(&mut self, p: Poem) {
        self.poems.insert(p.nn, p);
    }

    pub fn get_ordered_poems(&self) -> Vec<&Poem> {
        let mut vec = self.poems.values().collect_vec();
        vec.sort_by_key(|p| { &p.nn });
        // self.poems.values().into_iter().sorted_by(|a, b| Ord::cmp(&b.nn, &a.nn)).
        vec
    }
}

#[cfg(test)]
pub mod tests {
    use crate::poem::tests::get_test_poem;

    use super::*;

    pub fn get_test_book() -> Book {
        let mut res = Book::new(7);
        res.add(get_test_poem(5));
        res.add(get_test_poem(1));
        res.add(get_test_poem(2));
        res.add(get_test_poem(4));
        res.add(get_test_poem(7));
        res.add(get_test_poem(6));
        res.add(get_test_poem(3));
        res
    }

    #[test]
    fn test_sort() {
        let book = get_test_book();
        println!("{:#?}", book.get_ordered_poems());
        println!("{:#?}", book.get_ordered_poems());
        println!("{:#?}", book);
    }
}