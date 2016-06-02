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

pub fn parse_str(data: String) -> Result<ConfigBlock> {
    parse(data.chars())
}

pub fn parse_file(file: File) -> Result<ConfigBlock> {
    let s = String::new();
    file.read_to_string(&mut s);
    parse(s.chars())
}

#[cfg(test)]
mod tests {
    
}
