enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    // 没有关联任何数据。
    Move { x: i32, y: i32 },
    // 类似结构体包含命名字段。
    Write(String),
    // 包含单独一个 String。
    ChangeColor(i32, i32, i32), // 包含三个 i32。
}

impl Message {
    fn call(&self) {}
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    m.call();// 随便调用一下
}