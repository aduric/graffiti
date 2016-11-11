use std::collections::HashMap;
use std::str;
use std::fs;

use scanner::*;
use tokenizer::*;

pub struct Corpus {
    files: Vec<String>
}

impl Corpus {
    pub fn new(root_path: &str) -> Self {
        let paths = fs::read_dir(root_path).unwrap().map(|f| f.unwrap().file_name().into_string().unwrap()).collect::<Vec<String>>();
        Corpus {
            files: paths,
        }
    }
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
    pub fn words(&self, filename: &str) -> Vec<Vec<u8>> {
        let mut scanner = Scanner::new();
        let mut tokenizer = Tokenizer::new();

        scanner.scan(filename);
        let mut contents = scanner.get_contents();

        tokenizer.tokenize(contents);
        let mut tokens = tokenizer.get_tokens();

        let mut filtered_tokens = tokens.iter().filter(|t| t.1 == Token::Alpha).cloned().collect::<Vec<(Vec<u8>, Token)>>();
        filtered_tokens.into_iter().map(|t| t.0).collect::<Vec<Vec<u8>>>()        
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_files() {
        let corpus = Corpus::new("C:/Users/aduric/Downloads/brown/brown");

        let files = corpus.files();

        println!("{:?}", files);

        assert!(files.len() == 1024);
    }

    #[test]
    fn test_get_words() {
        let corpus = Corpus::new("C:/Users/aduric/Downloads/brown/brown");

        let words = corpus.words("ca01");

        assert!(words.len() == 1024);
    }
}