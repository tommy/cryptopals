#![allow(dead_code)]

use std::{
    fs::{self, read_to_string},
    io::{self, BufRead},
};

#[derive(Debug)]
enum Err {
    Io(io::Error),
    B64(base64::DecodeError),
}

impl From<io::Error> for Err {
    fn from(item: io::Error) -> Self {
        Self::Io(item)
    }
}

impl From<base64::DecodeError> for Err {
    fn from(item: base64::DecodeError) -> Self {
        Self::B64(item)
    }
}

pub fn file_b64(filename: &str) -> Vec<u8> {
    let contents: Vec<u8> = read_to_string(filename)
        .unwrap_or_else(|_| panic!("Couldn't read from {}", &filename))
        .as_bytes()
        .iter()
        .filter(|x| *x != &b'\n')
        .cloned()
        .collect();

    base64::decode(contents).expect("Couldn't base64 decode file")
}

#[test]
fn test_file_b64() {
    let s = file_b64("resources/test_file.txt");
    assert_eq!("hello world", std::str::from_utf8(&s).unwrap());
}

pub fn file_lines_b64(filename: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    io::BufReader::new(
        fs::File::open(filename).unwrap_or_else(|_| panic!("Couldn't read from {}", &filename)),
    )
    .lines()
    .map(|l| l.expect("not a utf8 string"))
    .map(|x| base64::decode(&x).expect("Couldn't decode base64"))
    .for_each(|bs| result.push(bs));

    result
}

#[test]
fn test_file_lines() {
    let ls = file_lines_b64("resources/8.txt");
    assert_eq!(
			base64::decode("8a10247f90d0a05538888ad6205882196f5f6d05c21ec8dca0cb0be02c3f8b09e382963f443aa514daa501257b09a36bf8c4c392d8ca1bf4395f0d5f2542148c7e5ff22237969874bf66cb85357ef99956accf13ba1af36ca7a91a50533c4d89b7353f908c5a166774293b0bf6247391df69c87dacc4125a99ec417221b58170e633381e3847c6b1c28dda2913c011e13fc4406f8fe73bbf78e803e1d995ce4d").unwrap(),
			ls[0]
		);
}

fn main() {}
