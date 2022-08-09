use cedict::DictEntry;

pub trait Entry {
    fn simplifed(&self) -> Option<&str>;
    fn traditional(&self) -> Option<&str>;
    fn pinyin(&self) -> &str;
    fn nth(&self, index: usize) -> Option<&str>;
}

pub struct CEEntry {
    base_entry: DictEntry,
}

impl From<DictEntry> for CEEntry {
    fn from(entry: DictEntry) -> CEEntry {
        CEEntry { base_entry: entry }
    }
}

//Adapt from CCEDICT Entry
impl Entry for CEEntry {
    fn simplifed(&self) -> Option<&str> {
        let simpl = self.base_entry.simplified();
         match simpl.is_empty() {
            true => None,
            false => Some(simpl),
        }
    }

    fn traditional(&self) -> Option<&str> {
        let trad = self.base_entry.traditional();
        match trad.is_empty() {
           true => None,
           false => Some(trad),
       }
    }

    fn pinyin(&self) -> &str {
        self.base_entry.pinyin()
    }

    fn nth(&self, index: usize) -> Option<&str> {
        self.base_entry.definitions().nth(index)
    }
}


pub struct BasicEntry {
    simplified: Option<String>,
    traditional: Option<String>,
    pinyin: String,
    definitions: Vec<String>,
}

impl BasicEntry
{
    
}

impl Entry for BasicEntry {
    fn simplifed(&self) -> Option<&str> {
        match &self.simplified {
            Some(string) => Some(string.as_str()),
            None => None,
        }
    }

    fn traditional(&self) -> Option<&str> {
        match &self.traditional {
            Some(string) => Some(string.as_str()),
            None => None,
        }
    }

    fn pinyin(&self) -> &str {
        self.pinyin.as_str()
    }

    fn nth(&self, index: usize) -> Option<&str> {
        match self.definitions.get(index)
        {
            Some(string) => Some(string.as_str()),
            None => None,
        }
        
    }
}
