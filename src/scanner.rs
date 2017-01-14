use std::io::{self, BufReader, Bytes};
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::str;

const DEFAULT_BUF_SIZE: usize = 64 * 1024;

pub struct Scanner {
        file: String,
}

impl Scanner {
        pub fn new(file: &str) -> Self {
                Scanner {
                        file: String::from(file),
                }
        }
        pub fn get_file(&self) -> &String {
                &self.file
        }
        pub fn scan(&self) -> Result<(Vec<u8>), io::Error> {
                let mut contents: Vec<u8> = Vec::new();

                let mut f = try!(File::open(&self.file));
                let mut buffer = [0; DEFAULT_BUF_SIZE];
                let mut total_count = 0;
                let mut data_size_within_buffer = 0;

                loop {
                        data_size_within_buffer = f.read(&mut buffer).unwrap();
                        for b in 0..data_size_within_buffer {
                                contents.push(buffer[b]);
                        }
                        if data_size_within_buffer < DEFAULT_BUF_SIZE {
                                break;
                        }
                } 

                Ok((contents))
        }
}

#[cfg(test)]
mod tests {

        use super::*;

        #[test]
        fn read_test_file_under_buffer_size() {
                let mut scanner = Scanner::new("./test_assets/foo.txt");

                let contents = scanner.scan().unwrap();

                assert_eq!(contents.len(), 3);
        }
}
