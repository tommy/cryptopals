#![allow(unused_imports)]

use std::{
    fs::{self, read_to_string},
    io::{self, BufRead},
    str::from_utf8,
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
    let ls: Vec<String> = file_lines_b64("resources/test_file_lines.txt")
        .iter()
        .map(|bs| from_utf8(bs).unwrap().to_owned())
        .collect();
    assert_eq!(vec!["hello world", "hello world"], ls);
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b)
        .map(|(ax, bx)| ax ^ bx)
        .map(|x| x.count_ones())
        .reduce(|x, y| x + y)
        .unwrap_or(0)
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    assert_eq!(hamming_distance(b"", b""), 0);
    assert_eq!(hamming_distance(b"asdf", b"asdf"), 0);
    assert_eq!(hamming_distance(b"foo", b"boo"), 1);
}

pub fn english_score(bs: &[u8]) -> u32 {
    const SPACE: u8 = 32;
    const TILDE: u8 = 126;
    const LOWER_A: u8 = 97;
    const LOWER_Z: u8 = 122;
    const ETAOIN: &[u8; 12] = b"etoainETOAIN";
    let mut in_range = 0;
    for b in bs {
        if SPACE <= *b && *b <= TILDE {
            in_range += 1;
        }
        if LOWER_A <= *b && *b <= LOWER_Z || *b == SPACE {
            in_range += 2;
        }
        if ETAOIN.contains(b) {
            in_range += 3;
        }
    }
    in_range
}
