use std::{
    error::{Error},
    path::{PathBuf}, io::ErrorKind::InvalidFilename,
};

use crate::chim_core::loaders::{loader::StringLoader, txt_loader::TxtLoader};

struct HTMLGen {}

impl HTMLGen {
    pub fn generate(path: &str, replacements: Vec<String>) -> Result<(), Box<dyn Error>> {
        let string = TxtLoader::load(path)? ;
        let generated_string = HTMLGen::replace(string, replacements);

        let mut new_path = PathBuf::from(path);
        match new_path.file_name() {
            Some(name) => {
                let new_name = name.to_str()
                .expect("Bizzarely, Failed to cast string.")
                .to_string();
                new_path.set_file_name(new_name + "_gen");
                Ok(std::fs::write(new_path, generated_string)?)
            }
            _ => Err(Box::new(std::io::Error::new(InvalidFilename, "Path must include filename."))),
        }
    }

    fn replace(string: String, replacements: Vec<String>) -> String {
        let mut to_replace = string;
        for r in replacements {
            to_replace = to_replace.replacen("{{}}", &r, 1);
        }

        to_replace
    }
}
