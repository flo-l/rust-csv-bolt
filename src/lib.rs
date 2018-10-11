#[cfg(test)]
extern crate itertools;

extern crate memchr;

use memchr::memchr3;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy)]
pub struct Csv<'a> {
    data: &'a [u8],
    row_sep: u8,
    col_sep: u8,
    quote: u8,
    escape: u8,
}

impl<'a> Csv<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Csv {
            data: bytes,
            row_sep: b'\n',
            col_sep: b',',
            quote: b'"',
            escape: b'\\',
        }
    }
}

impl<'a> Iterator for Csv<'a> {
    type Item = RowIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RowIter<'a> {
    row: &'a [u8],
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Cell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell<'a> {
    raw: &'a [u8],
}
