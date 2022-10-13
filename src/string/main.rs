// 字符串文本块
fn main() {
    let str = r#"
select *
from tb
"#;
    println!("{}", str);


    let s1 = String::from("Hello");
    let s2 = String::from(",World");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    let s4 = format!("{}-{}", s2, s3);
    println!("{}", s4);

}
