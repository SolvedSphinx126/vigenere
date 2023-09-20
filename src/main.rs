use std::collections::BinaryHeap;

// Define a static array of lowercase ASCII characters for reference.
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Define a static array of 998 common English words.
static WORDS: [&str; 998] = [
    "ability", "able", "about", "above", "accept", "according", "account", "across", "act",
    "action", "activity", "actually", "add", "address", "administration", "admit", "adult",
    "affect", "after", "again", "against", "age", "agency", "agent", "ago", "agree", "agreement",
    "ahead", "air", "all", "allow", "almost", "alone", "along", "already", "also", "although",
    "always", "American", "among", "amount", "analysis", "and", "animal", "another", "answer",
    "any", "anyone", "anything", "appear", "apply", "approach", "area", "argue", "arm", "around",
    "arrive", "art", "article", "artist", "as", "ask", "assume", "at", "attack", "attention",
    "attorney", "audience", "author", "authority", "available", "avoid", "away", "baby", "back",
    "bad", "bag", "ball", "bank", "bar", "base", "be", "beat", "beautiful", "because", "become",
    "bed", "before", "begin", "behavior", "behind", "believe", "benefit", "best", "better",
    "between", "beyond", "big", "bill", "billion", "bit", "black", "blood", "blue", "board",
    "body", "book", "born", "both", "box", "boy", "break", "bring", "brother", "budget", "build",
    "building", "business", "but", "buy", "by", "call", "camera", "campaign", "can", "cancer",
    "candidate", "capital", "car", "card", "care", "career", "carry", "case", "catch", "cause",
    "cell", "center", "central", "century", "certain", "certainly", "chair", "challenge", "chance",
    "change", "character", "charge", "check", "child", "choice", "choose", "church", "citizen",
    "city", "civil", "claim", "class", "clear", "clearly", "close", "coach", "cold", "collection",
    "college", "color", "come", "commercial", "common", "community", "company", "compare",
    "computer", "concern", "condition", "conference", "Congress", "consider", "consumer",
    "contain", "continue", "control", "cost", "could", "country", "couple", "course", "court",
    "cover", "create", "crime", "cultural", "culture", "cup", "current", "customer", "cut", "dark",
    "data", "daughter", "day", "dead", "deal", "death", "debate", "decade", "decide", "decision",
    "deep", "defense", "degree", "Democrat", "democratic", "describe", "design", "despite",
    "detail", "determine", "develop", "development", "die", "difference", "different", "difficult",
    "dinner", "direction", "director", "discover", "discuss", "discussion", "disease", "do",
    "doctor", "dog", "door", "down", "draw", "dream", "drive", "drop", "drug", "during", "each",
    "early", "east", "easy", "eat", "economic", "economy", "edge", "education", "effect", "effort",
    "eight", "either", "election", "else", "employee", "end", "energy", "enjoy", "enough", "enter",
    "entire", "environment", "environmental", "especially", "establish", "even", "evening",
    "event", "ever", "every", "everybody", "everyone", "everything", "evidence", "exactly",
    "example", "executive", "exist", "expect", "experience", "expert", "explain", "eye", "face",
    "fact", "factor", "fail", "fall", "family", "far", "fast", "father", "fear", "federal", "feel",
    "feeling", "few", "field", "fight", "figure", "fill", "film", "final", "finally", "financial",
    "find", "fine", "finger", "finish", "fire", "firm", "first", "fish", "five", "floor", "fly",
    "focus", "follow", "food", "foot", "for", "force", "foreign", "forget", "form", "former",
    "forward", "four", "free", "friend", "from", "front", "full", "fund", "future", "game",
    "garden", "gas", "general", "generation", "get", "girl", "give", "glass", "go", "goal", "good",
    "government", "great", "green", "ground", "group", "grow", "growth", "guess", "gun", "guy",
    "hair", "half", "hand", "hang", "happen", "happy", "hard", "have", "he", "head", "health",
    "hear", "heart", "heat", "heavy", "help", "her", "here", "herself", "high", "him", "himself",
    "his", "history", "hit", "hold", "home", "hope", "hospital", "hot", "hotel", "hour", "house",
    "how", "however", "huge", "human", "hundred", "husband", "idea", "identify", "if", "image",
    "imagine", "impact", "important", "improve", "in", "include", "including", "increase",
    "indeed", "indicate", "individual", "industry", "information", "inside", "instead",
    "institution", "interest", "interesting", "international", "interview", "into", "investment",
    "involve", "issue", "it", "item", "its", "itself", "job", "join", "just", "keep", "key", "kid",
    "kill", "kind", "kitchen", "know", "knowledge", "land", "language", "large", "last", "late",
    "later", "laugh", "law", "lawyer", "lay", "lead", "leader", "learn", "least", "leave", "left",
    "leg", "legal", "less", "let", "letter", "level", "lie", "life", "light", "like", "likely",
    "line", "list", "listen", "little", "live", "local", "long", "look", "lose", "loss", "lot",
    "love", "low", "machine", "magazine", "main", "maintain", "major", "majority", "make", "man",
    "manage", "management", "manager", "many", "market", "marriage", "material", "matter", "may",
    "maybe", "me", "mean", "measure", "media", "medical", "meet", "meeting", "member", "memory",
    "mention", "message", "method", "middle", "might", "military", "million", "mind", "minute",
    "miss", "mission", "model", "modern", "moment", "money", "month", "more", "morning", "most",
    "mother", "mouth", "move", "movement", "movie", "Mr", "Mrs", "much", "music", "must", "my",
    "myself", "name", "nation", "national", "natural", "nature", "near", "nearly", "necessary",
    "need", "network", "never", "new", "news", "newspaper", "next", "nice", "night", "no", "none",
    "nor", "north", "not", "note", "nothing", "notice", "now", "n't", "number", "occur", "of",
    "off", "offer", "office", "officer", "official", "often", "oh", "oil", "ok", "old", "on",
    "once", "one", "only", "onto", "open", "operation", "opportunity", "option", "or", "order",
    "organization", "other", "others", "our", "out", "outside", "over", "own", "owner", "page",
    "pain", "painting", "paper", "parent", "part", "participant", "particular", "particularly",
    "partner", "party", "pass", "past", "patient", "pattern", "pay", "peace", "people", "per",
    "perform", "performance", "perhaps", "period", "person", "personal", "phone", "physical",
    "pick", "picture", "piece", "place", "plan", "plant", "play", "player", "PM", "point",
    "police", "policy", "political", "politics", "poor", "popular", "population", "position",
    "positive", "possible", "power", "practice", "prepare", "present", "president", "pressure",
    "pretty", "prevent", "price", "private", "probably", "problem", "process", "produce",
    "product", "production", "professional", "professor", "program", "project", "property",
    "protect", "prove", "provide", "public", "pull", "purpose", "push", "put", "quality",
    "question", "quickly", "quite", "race", "radio", "raise", "range", "rate", "rather", "reach",
    "read", "ready", "real", "reality", "realize", "really", "reason", "receive", "recent",
    "recently", "recognize", "record", "red", "reduce", "reflect", "region", "relate",
    "relationship", "religious", "remain", "remember", "remove", "report", "represent",
    "Republican", "require", "research", "resource", "respond", "response", "responsibility",
    "rest", "result", "return", "reveal", "rich", "right", "rise", "risk", "road", "rock", "role",
    "room", "rule", "run", "safe", "same", "save", "say", "scene", "school", "science",
    "scientist", "score", "sea", "season", "seat", "second", "section", "security", "see", "seek",
    "seem", "sell", "send", "senior", "sense", "series", "serious", "serve", "service", "set",
    "seven", "several", "sex", "sexual", "shake", "share", "she", "shoot", "short", "shot",
    "should", "shoulder", "show", "side", "sign", "significant", "similar", "simple", "simply",
    "since", "sing", "single", "sister", "sit", "site", "situation", "six", "size", "skill",
    "skin", "small", "smile", "so", "social", "society", "soldier", "some", "somebody", "someone",
    "something", "sometimes", "son", "song", "soon", "sort", "sound", "source", "south",
    "southern", "space", "speak", "special", "specific", "speech", "spend", "sport", "spring",
    "staff", "stage", "stand", "standard", "star", "start", "state", "statement", "station",
    "stay", "step", "still", "stock", "stop", "store", "story", "strategy", "street", "strong",
    "structure", "student", "study", "stuff", "style", "subject", "success", "successful", "such",
    "suddenly", "suffer", "suggest", "summer", "support", "sure", "surface", "system", "table",
    "take", "talk", "task", "tax", "teach", "teacher", "team", "technology", "television", "tell",
    "ten", "tend", "term", "test", "than", "thank", "that", "the", "their", "them", "themselves",
    "then", "theory", "there", "these", "they", "thing", "think", "third", "this", "those",
    "though", "thought", "thousand", "threat", "three", "through", "throughout", "throw", "thus",
    "time", "to", "today", "together", "tonight", "too", "top", "total", "tough", "toward", "town",
    "trade", "traditional", "training", "travel", "treat", "treatment", "tree", "trial", "trip",
    "trouble", "true", "truth", "try", "turn", "TV", "two", "type", "under", "understand", "unit",
    "until", "up", "upon", "us", "use", "usually", "value", "various", "very", "victim", "view",
    "violence", "visit", "voice", "vote", "wait", "walk", "wall", "want", "war", "watch", "water",
    "way", "we", "weapon", "wear", "week", "weight", "well", "west", "western", "what", "whatever",
    "when", "where", "whether", "which", "while", "white", "who", "whole", "whom", "whose", "why",
    "wide", "wife", "will", "win", "wind", "window", "wish", "with", "within", "without", "woman",
    "wonder", "word", "work", "worker", "world", "worry", "would", "write", "writer", "wrong",
    "yard", "yeah", "year", "yes", "yet", "you", "young", "your", "yourself",
];

fn main() {
    // Define a plaintext message and a key.
    let plaintext = "thequickbrownfoxjumpedoverthelazydog".to_string();
    let key = "gut".to_string();

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
                for i in 0..998 {
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
            "{}",
            format!(
                "{:2}: {} decoded with key {} had {} overlapping matches",
                possible_break.0, possible_break.1 .2, possible_break.1 .3, possible_break.1 .0
            )
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
