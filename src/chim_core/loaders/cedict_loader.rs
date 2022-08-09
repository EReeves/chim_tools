
use std::{collections::HashMap};
use crate::chim_core::{entry::CEEntry};

use super::loader::EntryLoader;

pub struct CEDictLoader {}

impl EntryLoader<CEEntry> for CEDictLoader
{
    fn load(path: &str) -> Result<HashMap<String, CEEntry>, std::io::Error>
    {
        let file = std::fs::File::open(path)?;
        let parsed = cedict::parse_reader(file);
    
        Ok(parsed
            .map(|x| -> (String, CEEntry) {(x.simplified().to_string(), crate::chim_core::entry::CEEntry::from(x))})
            .collect())
    }
}