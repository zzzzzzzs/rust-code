fn main() {
    let str = String::from("Hello World!");
    let hello = &str[0..5];
    let world = &str[6..11];
    println!("{}", hello);
}

fn first_word(str: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}