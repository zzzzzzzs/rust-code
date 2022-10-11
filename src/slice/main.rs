fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{}", word);
    s.clear(); // 清空字符串，相当于 ""
    println!("{}", s);
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let word1 = second_word(&s);
    println!("{}", word1);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}