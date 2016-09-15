use std::{iter, slice};

/// Represents a block in the config file or the document root
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigBlock {
    name: String,
    params: Vec<String>,
    inner: Vec<ConfigBlock>
}

impl ConfigBlock {
    /// Creates a new ConfigBlock. This probably isn't very useful to you.
    pub fn new(name: String, params: Vec<String>, inner: Vec<ConfigBlock>) -> ConfigBlock {
        ConfigBlock {
            name: name,
            params: params,
            inner: inner
        }
    }

    /// Adds a new sub block. This probably isn't very useful for you
    pub fn add_block(&mut self, block: ConfigBlock) {
        self.inner.push(block);
    }

    /// Returns an iterator of all inner config options with the specified name
    pub fn matching<'a>(&'a self, name: &'a str) -> ConfigIter<'a> {
        ConfigIter {
            it: self.inner.iter(),
            name: name
        }
    }

    /// Returns the name of the option key
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all inner config options
    pub fn inner<'a>(&'a self) -> &'a Vec<ConfigBlock> {
        &self.inner
    }

    /// Returns the parameter count
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Returns a parameter. Panics if the parameter index is too high
    pub fn get(&self, i: usize) -> &str {
        &self.params[i]
    }

    /// Returns a parameter. Returns None if the index is too high
    pub fn get_opt(&self, i: usize) -> Option<&str> {
        if i < self.params.len() {
            Some(&self.params[i])
        } else {
            None
        }
    }
}

pub struct ConfigIter<'a> {
    it: slice::Iter<'a, ConfigBlock>,
    name: &'a str
}

impl <'a> iter::Iterator for ConfigIter <'a> {
    type Item = &'a ConfigBlock;
    fn next(&mut self) -> Option<&'a ConfigBlock> {
        loop {
            match self.it.next() {
                Some(c) if c.name() == self.name => return Some(c),
                Some(_) => {},
                None => return None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.it.size_hint().1)
    }
}
