#[allow(dead_code)]
pub fn xor(bs: &[u8], key: u8) -> Vec<u8> {
    bs.iter().map(|b| b ^ key).collect()
}

#[allow(dead_code)]
pub fn break_single_byte_xor_to_string(bs: &[u8]) -> Option<String> {
    (0x00..=0xFF)
        .into_iter()
        .map(|k| xor(bs, k))
        .max_by_key(|c| cryptopals::english_score(c))
        .as_ref()
        .and_then(|c| std::str::from_utf8(c).ok())
        .map(|s| s.to_owned())
}

#[allow(dead_code)]
pub fn break_single_byte_xor(bs: &[u8]) -> (Vec<u8>, u8) {
    let mut best: (Vec<u8>, u8, u32) = (vec![], 0, 0);

    for k in 0x00..=0xFF {
        let candidate = xor(bs, k);
        let score = cryptopals::english_score(&candidate);
        if score > best.2 {
            best = (candidate, k, score);
        }
    }

    (best.0, best.1)
}

#[test]
fn test_1_3() {
    assert_eq!(
        break_single_byte_xor_to_string(
            &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap()
        )
        .unwrap(),
        "Cooking MC's like a pound of bacon"
    );

    assert_eq!(
        break_single_byte_xor(
            &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap()
        ),
        ("Cooking MC's like a pound of bacon".as_bytes().to_vec(), 88)
    );
}

#[allow(dead_code)]
fn main() {}
