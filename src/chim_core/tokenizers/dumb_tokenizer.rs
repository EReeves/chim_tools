use std::error::Error;

use super::tokenizer::{Tokenizer};

#[derive(Default)]
pub struct DumbTokenizer {}

impl Tokenizer for DumbTokenizer {
    fn tokenize(&self, cntext: &String) -> Result<Vec<u8>, Box<dyn Error>>  {

        //Every character is a word accoridng to this tokenizer, so just 1s
        let vec = vec![1u8; cntext.len()];
        Ok(vec)
    }
}
