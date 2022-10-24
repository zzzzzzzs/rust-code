use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use json::JsonValue::String;

// 为应对panic，展开或中止 (abort）调用栈。
// 默认情况下，当panic 发生:
// 1、程序展开调用栈（工作量大)
// Rust沿着调用栈往回走
// 清理每个遇到的函数中的数据
// 2、立即中止调用栈:
// 不进行清理，直接停止程序
// 内存需要OS进行清理
// 想让二进制文件更小，把设置从“展开”改为“中止”:
// panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'
fn main() {
    let f = File::open("hello.txt");

    // 原始的方法
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file : {:?}", e),
            },
            oe => panic!("Error opening the file: {:?}", oe),
        }
    };

    // 新的方式
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    // unwrap 是 match 表达式的一个快捷方法，报错的信息无法自定义
    let f = File::open("hello.txt").unwrap();

    // expect 可以自定义信息
    let f = File::open("hello.txt").expect("无法打开文件");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 运算符，只能适用于返回 Result 的函数，传播错误的一种快捷方式
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}