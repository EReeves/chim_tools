use std::collections::HashMap;

use crate::chim_core::entry::Entry;


pub trait StringLoader{
    fn load(path: &str) -> Result<String, std::io::Error>;
}

pub trait EntryLoader<T: Entry>{
    fn load(path: &str) -> Result<HashMap<String, T>, std::io::Error>;
}


//Maybe a generator type as well?