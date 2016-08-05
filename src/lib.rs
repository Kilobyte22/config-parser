#![doc(html_root_url = "https://kilobyte22.de/doc/config_parser/")]

pub mod config;
pub mod error;
pub mod lexer;
pub mod parser;

pub use config::ConfigBlock;
pub use error::{Result, Error as ParseError};

use std::fs::File;
use std::io::Read;

/// Parses a configuration file by an iterator of chars
pub fn parse<T, I>(iter: T) -> Result<ConfigBlock> where 
        T: IntoIterator<Item=char, IntoIter=I> + Sized,
        I: Iterator<Item=char> + 'static {
    parser::run(Box::new(try!(lexer::run(Box::new(iter.into_iter()))).into_iter()))
}

pub fn parse_string(data: String) -> Result<ConfigBlock> {
    parse(OwningChars::new(data))
}

pub fn parse_file(mut file: File) -> Result<ConfigBlock> {
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    parse_string(s)
}

struct OwningChars { s: String, pos: usize }

impl OwningChars {
    pub fn new(s: String) -> OwningChars {
        OwningChars { s: s, pos: 0 }
    }
}

impl Iterator for OwningChars {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.s[self.pos..].chars().next() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.s.len() - self.pos;
        ((len + 3) / 4, Some(len)) // see the Chars impl for detail
    }
}

#[cfg(test)]
mod tests {
    
}
