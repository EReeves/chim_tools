use std::error::Error;

use num::clamp;
use crate::chim_core::{
    entry::CEEntry,
    loaders::{cedict_loader::CEDictLoader, loader::EntryLoader},
};
use super::tokenizer::Tokenizer;

//Algorithm based loosely on https://github.com/yishn/chinese-tokenizer and requires CC-CCEDICT in /res folder
//Simple, it just looks up the longest word in CC-CCEDICT
pub struct GreedyTokenizer {
    look_ahead: usize,
}

impl Default for GreedyTokenizer {
    fn default() -> Self {
        GreedyTokenizer {
            look_ahead: 6,
        }
    }
}

impl Tokenizer for GreedyTokenizer {
    fn tokenize(&self, cntext: &String) -> Result<Vec<u8>, Box<dyn Error>> {
        let hm = CEDictLoader::load("./res/cedict_1_0_ts_utf-8_mdbg.txt")?;
        let mut tokens = Vec::<u8>::new();

        let big_slice = cntext.as_str();
        let real_length = cntext.chars().count();
        let look_ahead_max = clamp(self.look_ahead, 1, real_length);
        let mut pos = 0;

        while pos < cntext.chars().count() {
            //get slice bounds
            let mut look_ahead = pos + look_ahead_max;
            if look_ahead > real_length {
                look_ahead = real_length
            }

            //Slice and check for match
            let mut pot = utf8_slice::slice(big_slice, pos, look_ahead + 1);
            let mut hres: Option<&CEEntry> = None;

            while let None = hres {
                pot = utf8_slice::slice(pot, 0, pot.chars().count() - 1);
                if pot.chars().count() <= 1 {
                    break;
                }
                hres = hm.get(pot);
            }

            if pot.is_empty() {
                pos += 1;
                continue;
            }

            let num_chars = pot.chars().count();
            tokens.push(num_chars as u8);
            pos += num_chars;
        }

        Ok(tokens)
    }
}
