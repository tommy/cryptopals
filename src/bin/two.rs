#[allow(dead_code)]
pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (ia, ib) in a.iter().zip(b.iter()) {
        result.push(ia ^ ib);
    }
    result
}

#[allow(dead_code)]
pub fn xor_two(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(ia, ib)| ia ^ ib).collect()
}

#[test]
fn test_main() {
    assert_eq!(
        hex::encode(xor_two(
            hex::decode("1c0111001f010100061a024b53535009181c")
                .unwrap()
                .as_slice(),
            hex::decode("686974207468652062756c6c277320657965")
                .unwrap()
                .as_slice()
        )),
        "746865206b696420646f6e277420706c6179"
    );
}

fn main() {}
