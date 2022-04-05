#![allow(dead_code)]

use openssl::symm::{encrypt, Cipher};
use rand::{distributions::Standard, Rng};
mod eight;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BlockMode {
    Ecb,
    Cbc,
}

fn random_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(5..=10);

    (&mut rng).sample_iter(Standard).take(n).collect()
}

#[test]
fn test_random_bytes() {
    let x: Vec<u8> = random_bytes();
    assert!(5 <= x.len() && x.len() <= 10);
}

fn encryption_oracle(mode: BlockMode) -> impl Fn(&[u8]) -> Vec<u8> {
    move |s: &[u8]| {
        let ecb = mode == BlockMode::Ecb;
        let key = rand::random::<[u8; 16]>();

        let cipher = if ecb {
            Cipher::aes_128_ecb()
        } else {
            Cipher::aes_128_cbc()
        };

        let iv = rand::random::<[u8; 16]>();
        let iv = if ecb { None } else { Some(iv.as_ref()) };

        let bs = [random_bytes(), s.to_vec(), random_bytes()].concat();

        encrypt(cipher, &key, iv, &bs).expect("Could not encrypt!")
    }
}

fn detect_ecb(oracle: &dyn Fn(&[u8]) -> Vec<u8>) -> BlockMode {
    let plaintext = ["a".as_bytes()[0]; 16 * 20];
    if eight::detect_ecb(&oracle(&plaintext)) > 0 {
        BlockMode::Ecb
    } else {
        BlockMode::Cbc
    }
}

#[test]
fn test_detect_ecb() {
    let mode = if rand::random::<bool>() {
        BlockMode::Ecb
    } else {
        BlockMode::Cbc
    };
    let x = detect_ecb(&encryption_oracle(mode));
    print!("{:?}, {:?}", mode, x);
    assert_eq!(x, mode);
}

fn main() {
    // print!("{:?}", encryption_oracle("foo"));
}
