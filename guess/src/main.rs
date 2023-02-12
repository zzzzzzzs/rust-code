use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        // String 继承的是 vec，read_line 每次都会往 guess 中添加一个元素
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // parse 将字符串转化成整数
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => {
            Err(err) => {
                //  目的是为了打印异常问题
                println!("出现异常{}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // guess 数字和 secret_num 数字进行比较
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}