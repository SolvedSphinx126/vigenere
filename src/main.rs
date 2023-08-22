static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
fn main() {
    let plaintext = "theboyhastheball".to_string().to_ascii_lowercase();
    let key = "vig".to_string().to_ascii_lowercase();
    let ciphertext = "OPKWWECIYOPKWIRG".to_string().to_ascii_lowercase();
    println!("{}", encode(&plaintext, &key));
    println!("{}", decode(&ciphertext, &key));
}

fn encode(str: &String, key: &String) -> String {
    let mut key = key.chars().into_iter().cycle();
    let mut str = str.chars().into_iter();
    let mut cipher_text = String::new();
    for chr in str {
        cipher_text.push(add(chr, key.next().unwrap()))
    }
    cipher_text
}

fn decode(str: &String, key: &String) -> String {
    let mut key = key.chars().into_iter().cycle();
    let mut str = str.chars().into_iter();
    let mut cipher_text = String::new();
    for chr in str {
        cipher_text.push(sub(chr, key.next().unwrap()))
    }
    cipher_text
}

// Declare trait


fn add(lhs: char, rhs: char) -> char {
    // ensure that the characters are both valid for the addition operation
    assert!(lhs.is_ascii_alphabetic());
    assert!(rhs.is_ascii_alphabetic());
    assert!(lhs.is_ascii_lowercase());
    assert!(rhs.is_ascii_lowercase());
    let lhs_index = ASCII_LOWER.iter().position(|chr| *chr == lhs).unwrap();
    let rhs_index = ASCII_LOWER.iter().position(|chr| *chr == rhs).unwrap();
    ASCII_LOWER[(lhs_index + rhs_index) % 26]
}

fn sub(lhs: char, rhs: char) -> char {
    // ensure that the characters are both valid for the subtraction operation
    assert!(lhs.is_ascii_alphabetic());
    assert!(rhs.is_ascii_alphabetic());
    assert!(lhs.is_ascii_lowercase());
    assert!(rhs.is_ascii_lowercase());
    let lhs_index = ASCII_LOWER.iter().position(|chr| *chr == lhs).unwrap();
    let rhs_index = ASCII_LOWER.iter().position(|chr| *chr == rhs).unwrap();
    ASCII_LOWER[((lhs_index + 26) - rhs_index) % 26]
}
