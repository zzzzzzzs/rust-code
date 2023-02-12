use std::io;

fn main() {
    println!("输入一个数字！");

    // let mut foo = 1;
    // let bar = foo; // immutable
    // foo = 2;
    let mut guess = String::new();

    // io::Result Ok, Err
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("输出的数字：{}", guess);
}