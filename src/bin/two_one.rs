#![allow(dead_code)]

fn pad_pkcs7(n: usize, bs: &[u8]) -> Vec<u8> {
    let n_pad = n - bs.len() % n;
    let ns = [n_pad as u8].repeat(n_pad);

    [bs, &ns].concat()
}

#[test]
fn test_pad_pkcs() {
    assert_eq!(
        "YELLOW SUBMARINE\x04\x04\x04\x04",
        std::str::from_utf8(&pad_pkcs7(20, "YELLOW SUBMARINE".as_bytes())).unwrap()
    )
}

fn main() {}
