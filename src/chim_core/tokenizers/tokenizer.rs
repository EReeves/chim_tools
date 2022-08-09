use std::error::Error;


//Can split CN into words
pub trait Tokenizer {

    ///Return a vec of u8s with position of end of each word (relative to last)
    ///e.g. 你非常棒 = 1, 2, 1 (with perfect tokenization)
    fn tokenize(&self, cntext: &String) -> Result<Vec<u8>, Box<dyn Error>>;
}
