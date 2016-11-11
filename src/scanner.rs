use std::io::{self, BufReader, Bytes};
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::str;

const DEFAULT_BUF_SIZE: usize = 64 * 1024;

pub struct Scanner{
        contents: Vec<u8>,
}

impl Scanner {
        pub fn new() -> Self {
                Scanner {
                        contents: Vec::new(),
                }
        }
        pub fn get_contents(&mut self) -> &Vec<u8> {
                &self.contents
        }
        pub fn get_contents_size(&mut self) -> usize {
                self.contents.len()
        }
        pub fn scan(&mut self, file: &str) -> Result<(), io::Error> {
                let mut f = try!(File::open(file));
                let mut buffer = [0; DEFAULT_BUF_SIZE];
                let mut total_count = 0;
                let mut data_size_within_buffer = 0;

                loop {
                        data_size_within_buffer = f.read(&mut buffer).unwrap();
                        for b in 0..data_size_within_buffer {
                                self.contents.push(buffer[b]);
                        }
                        if data_size_within_buffer < DEFAULT_BUF_SIZE { break; }
                } 

                Ok(())
        }
}

#[cfg(test)]
mod tests {

        use super::*;

        #[test]
        fn read_test_file_under_buffer_size() {
                let mut scanner = Scanner::new();

                scanner.scan("C:/seen/foo.txt");

                assert_eq!(scanner.get_contents_size(), 3);
        }
}
