use std::collections::BinaryHeap;

// Define a static array of lowercase ASCII characters for reference.
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Define a static array of 100 common English words.
static WORDS: [&str; 100] = [
    "the", "at", "there", "some", "my", "of", "be", "use", "her", "than", "and", "this", "an",
    "would", "first", "a", "have", "each", "make", "water", "to", "from", "which", "like", "been",
    "in", "or", "she", "him", "call", "is", "one", "do", "into", "who", "you", "had", "how",
    "time", "oil", "that", "by", "their", "has", "its", "it", "word", "if", "look", "now", "he",
    "but", "will", "two", "find", "was", "not", "up", "more", "long", "for", "what", "other",
    "write", "down", "on", "all", "about", "go", "day", "are", "were", "out", "see", "did", "as",
    "we", "many", "number", "get", "with", "when", "then", "no", "come", "his", "your", "them",
    "way", "made", "they", "can", "these", "could", "may", "I", "said", "so", "people", "part",
];

fn main() {
    // Define a plaintext message and a key.
    let plaintext = "thecatrandowntheroad".to_string();
    let key = "fgi".to_string();

    // Encode the plaintext using the key and print the result.
    let ciphertext = encode(&plaintext, &key).to_string();
    println!("Encoded string is: {}", encode(&plaintext, &key));

    // Decode the ciphertext using the key and print the result.
    println!("Decoded string is: {}", decode(&ciphertext, &key));

    println!("");
    println!("The following are the most likely keys and decoded messages");

    // Attempt to break the cipher and print possible key candidates.
    break_cipher(&&ciphertext.to_string())
}

// Define a function to encode a string using a key using vigenere cipher.
fn encode(str: &String, key: &String) -> String {
    // Convert the key to lowercase.
    let binding = key.to_ascii_lowercase();

    // Create an iterator for the key characters that cycles indefinitely.
    let mut key = binding.chars().into_iter().cycle();

    // Create an iterator for the string characters.
    let str = str.chars().into_iter();
    let mut cipher_text = String::new();

    // Iterate over the characters in the string add each one with the corresponding key character
    for chr in str {
        cipher_text.push(add(chr, key.next().unwrap()))
    }

    // return cipher text
    cipher_text
}

// Define a function to decode a string using a key.
fn decode(str: &String, key: &String) -> String {
    // Create an iterator for the key characters that cycles indefinitely.
    let mut key = key.chars().into_iter().cycle();

    // Convert the string to lowercase.
    let binding = str.to_ascii_lowercase();

    // Create an iterator for the string characters.
    let str = binding.chars().into_iter();
    let mut cipher_text = String::new();

    // Iterate over the characters in the string subtract each one with the corresponding key character
    for chr in str {
        cipher_text.push(sub(chr, key.next().unwrap()))
    }

    // return cipher text
    cipher_text
}

// Define a function to add two lowercase alphabetical characters.
fn add(lhs: char, rhs: char) -> char {
    // Ensure that the characters are both valid for the addition operation.
    assert!(lhs.is_ascii_alphabetic());
    assert!(rhs.is_ascii_alphabetic());
    assert!(lhs.is_ascii_lowercase());
    assert!(rhs.is_ascii_lowercase());

    // Find the index of the characters in the ASCII_LOWER array and calculate the result.
    let lhs_index = ASCII_LOWER.iter().position(|chr| *chr == lhs).unwrap();
    let rhs_index = ASCII_LOWER.iter().position(|chr| *chr == rhs).unwrap();

    // return the new character
    ASCII_LOWER[(lhs_index + rhs_index) % 26]
}

// Define a function to subtract two lowercase alphabetical characters.
fn sub(lhs: char, rhs: char) -> char {
    // Ensure that the characters are both valid for the subtraction operation
    assert!(lhs.is_ascii_alphabetic());
    assert!(rhs.is_ascii_alphabetic());
    assert!(lhs.is_ascii_lowercase());
    assert!(rhs.is_ascii_lowercase());

    // Find the index of the characters in the ASCII_LOWER array and calculate the result.
    let lhs_index = ASCII_LOWER.iter().position(|chr| *chr == lhs).unwrap();
    let rhs_index = ASCII_LOWER.iter().position(|chr| *chr == rhs).unwrap();

    // return the new character
    ASCII_LOWER[((lhs_index + 26) - rhs_index) % 26]
}

// Define a function to attempt to break the cipher.
fn break_cipher(ciphertext: &String) {
    // Initialize a key for decryption.
    let mut key = "aaa".to_string();

    // Initialize a leaderboard to keep track of potential key candidates and their scores.
    let mut leaderboard = BinaryHeap::<(u32, String, String, String)>::new();
    for _ in 0..26 {
        for _ in 0..26 {
            for _ in 0..26 {
                // for each key in the keyspace
                let mut score = 0;

                // Decode the ciphertext using the current key and count total word occurances
                for i in 0..100 {
                    score += decode(ciphertext, &key).matches(WORDS[i]).count();
                }

                // Add the score and relevant information to the leaderboard.
                leaderboard.push((
                    score.try_into().unwrap(),
                    ciphertext.clone(),
                    decode(&ciphertext.clone(), &key),
                    key.clone(),
                ));

                // Increment the key.
                key = increment_string(&key);
            }
        }
    }

    // Print the top 30 potential key candidates along with their scores.
    for possible_break in leaderboard
        .into_sorted_vec()
        .into_iter()
        .rev()
        .take(30)
        .enumerate()
    {
        println!(
            "{}", format!("{:2}: {} decoded with key {} had {} overlapping matches", possible_break.0, possible_break.1 .2, possible_break.1 .3, possible_break.1 .0)
        );
    }
    
}

// Define a function to increment a string.
fn increment_string(str: &String) -> String {
    // Create a reverse iterator for the characters in the input string.
    let reverse_iterator = str.chars().into_iter().rev();

    // Initialize an empty string to store the result.
    let mut ret = String::new();

    // Initialize a boolean variable to keep track of whether there is a carry.
    let mut carry = true;

    // Iterate over the characters in reverse order (from right to left).
    for c in reverse_iterator {
        // If there is a carry from the previous character addition, add 'b' to the current character.
        // Otherwise, add the current character as is.
        if carry {
            ret.push(add(c, 'b'));
        } else {
            ret.push(c);
        }

        // Check if there is a carry for the next iteration:
        // The carry occurs if the last character added to 'ret' is 'a' and was carried.
        carry = carry & (ret.clone().chars().last().unwrap() == 'a')
    }

    // Reverse the characters in 'ret' and collect them into a String to get the final result.
    ret.chars().into_iter().rev().collect()
}
