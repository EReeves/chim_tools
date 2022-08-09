

use chimtool::chim_core::tokenizers::greedy_tokenizer::GreedyTokenizer;
use chimtool::chim_core::{
    loaders::{loader::StringLoader, txt_loader::TxtLoader},
    tokenizers::tokenizer::Tokenizer,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of file to parse
    filename: String,
}

fn main() {
    //let _args = Args::parse();

    let res = TxtLoader::load("./res/test.txt");//args.filename);
    let string = res.unwrap();
    let tokenizer = GreedyTokenizer::default();
    let tokens = tokenizer.tokenize(&string).expect("couldn't get tokens");

    println!("Count: {}\nTokens:", tokens.len());


    for tk in &tokens {
        print!("{} ", tk);
    }

    println!("");

    let str = string.as_str();

    let mut acc = 0;
    for tk in tokens {
        let to_write = utf8_slice::slice(str, acc, acc + tk as usize);
        print!("{}, ", to_write);
        acc += tk as usize;
    }

 
}
