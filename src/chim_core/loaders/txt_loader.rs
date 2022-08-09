
use std::fs;
use super::loader::StringLoader;

pub struct TxtLoader {}

impl StringLoader for TxtLoader
{
    fn load(path: &str) -> Result<String, std::io::Error>
    {
       fs::read_to_string(path)
    }
}