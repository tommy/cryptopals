#![allow(dead_code)]

mod five;
mod three;

fn score_keysize(bs: &[u8], keysize: usize) -> f64 {
    let (one, two) = (bs.chunks_exact(keysize), bs.chunks_exact(keysize));
    let scores: Vec<u32> = one
        .zip(two.skip(1))
        .map(|(a, b)| cryptopals::hamming_distance(a, b))
        .collect();
    let len = scores.len();
    let sum: u32 = scores.iter().sum();
    sum as f64 / len as f64 / keysize as f64
}

#[test]
fn test_score_keysize() {
    assert!(
        (score_keysize(b"this is a testwokka wokka!!!", 14) - 37.0 / 14.0).abs() < f64::EPSILON
    );
}

fn best_keysizes(bs: &[u8]) -> Vec<usize> {
    let mut keysizes: Vec<usize> = (2..=40).collect();
    keysizes.sort_by(|a, b| {
        score_keysize(bs, *a)
            .partial_cmp(&score_keysize(bs, *b))
            .unwrap()
    });

    // println!(
    //     "{:#?}",
    //     keysizes
    //         .iter()
    //         .map(|k| (k, score_keysize(bs, *k)))
    //         .collect::<Vec<(&usize, f64)>>()
    // );

    keysizes
}

fn split_and_transpose(bs: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![Vec::new(); keysize];

    for (i, b) in bs.iter().enumerate() {
        result[i % keysize].push(*b);
    }

    result
}

#[test]
fn test_split_and_transpose() {
    assert_eq!(split_and_transpose(b"abcdef", 2), vec![b"ace", b"bdf"]);
}

fn crack(bs: &[u8], keysize: usize) -> (Vec<u8>, Vec<u8>) {
    let transposed = split_and_transpose(bs, keysize);
    let mut result = Vec::new();
    let mut key = Vec::new();
    for block in transposed {
        let mut cracked_block = three::break_single_byte_xor(block.as_slice());
        result.append(&mut cracked_block.0);
        key.push(cracked_block.1);
    }
    (result, key)
}

#[test]
fn test_crack() {
    let (result, _key) = crack(
        &hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap(),
        1,
    );
    println!("{:?}", std::str::from_utf8(&result));
    assert_eq!(
        Ok("Cooking MC's like a pound of bacon"),
        std::str::from_utf8(&result)
    )
}

// bad (13): [105, 110, 105, 105, 110, 105, 110, 111, 105, 110, 114, 105, 105]
// good (29): [84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103, 32, 116, 104, 101, 32, 110, 111, 105, 115, 101]

#[test]
fn test_foo() {
    println!(
        "{}",
        vec![
            84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103,
            32, 116, 104, 101, 32, 110, 111, 105, 115, 101
        ]
        .len()
    )
}

#[allow(dead_code)]
fn crack_repeating_key_xor(bs: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let a = best_keysizes(bs)
        .iter()
        .take(4)
        .map(|keysize| crack(bs, *keysize))
        .max_by_key(|(s, _key)| cryptopals::english_score(s));

    if let Some((_, key)) = a {
        let decrypted = five::repeating_key_xor(bs, &key);
        Some((decrypted, key))
    } else {
        None
    }
}

#[test]
fn test_crack_repeating_key_xor() {
    let f = cryptopals::file_b64("resources/6.txt");
    let (decrypted, key) = crack_repeating_key_xor(&f).unwrap();
    assert_eq!(
        "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n",
        std::str::from_utf8(&decrypted).unwrap()
    );
    assert_eq!(
        vec![
            84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103,
            32, 116, 104, 101, 32, 110, 111, 105, 115, 101
        ],
        key
    )
}

fn main() {
    let f = cryptopals::file_b64("resources/6.txt");
    let a = best_keysizes(&f)
        .iter()
        .take(4)
        .map(|keysize| crack(&f, *keysize))
        .max_by_key(|s| cryptopals::english_score(&s.0));
    if let Some((_bs, key)) = a {
        let decrypted = five::repeating_key_xor(&f, &key);
        println!("{:?}", std::str::from_utf8(&decrypted));
        println!("{:?}", key);
    }
}
