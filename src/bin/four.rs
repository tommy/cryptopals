mod three;

use std::fs::File;
use std::io::{self, BufRead};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("resources/4.txt")?;
    Ok(io::BufReader::new(file).lines())
}

fn find_single_byte_xor() -> io::Result<String> {
    let lines = read_lines()?;
    Ok(lines
        .flatten()
        .map(hex::decode)
        .map(|bs| bs.unwrap())
        .map(|l| three::break_single_byte_xor_to_string(&l))
        .flatten()
        .max_by_key(|f| cryptopals::english_score(f.as_bytes()))
        .expect("Could not find any solution"))
}

#[test]
fn test_1_4() {
    assert_eq!(
        find_single_byte_xor().unwrap(),
        "Now that the party is jumping\n"
    );
}

#[allow(dead_code)]
fn main() {
    println!("{}", find_single_byte_xor().unwrap());
}
