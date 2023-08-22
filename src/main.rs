fn main() {
    let str = "Testing".to_string();
    let key = "abc".to_string();
    encode(&str, &key);
    println!("Hello, world!");
}

fn encode(str: &String, key: &String) -> String {
    let mut key = key.chars().into_iter().cycle();
    let var = key.take(10).collect::<String>();
    dbg!(var);
    todo!();
}