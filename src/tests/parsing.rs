#[cfg(test)]

#[test]
fn it_tokenizes()
{
    use crate::chim_core::tokenizers::{dumb_tokenizer::DumbTokenizer, tokenizer::Tokenizer};

    let str = "这是测试".to_string();

    let res = DumbTokenizer::default().tokenize(&str).expect("fail");
    println!("{:?}, words: {}", res, res.len());
    assert_eq!(res.len() > 0, true);
}