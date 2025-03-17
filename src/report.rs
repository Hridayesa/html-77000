// use serde::{Deserialize, Serialize};

use std::fmt::format;

use crate::{Result, Error};

#[derive(Debug)]
pub struct BookReport{
    nn: u32,
    errors: Vec<String>
}
impl BookReport {
    pub(crate) fn new(nn: u32) -> Self {
        Self { 
            nn,
            errors: Default::default(),
         }
    }
    
    // pub fn add<E: Into<Error>>(&mut self, error: E){
    //     self.errors.push(format!("{:?}", error.into()));
    // }

    pub fn add(&mut self, error: &Error){
        self.errors.push(format!("{:?}", error));
    }
}