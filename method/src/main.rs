use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块 使得定义的函数在 Rectangle 中
impl Rectangle {
    // &self 是 selt: & Self 的缩写s
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 判断一个长方形能包含另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数，使用的时候用 ::
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// TODO 将求长方形面积的函数变成方法
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("{:#?}", rect1);
    println!("{}", rect1.area());
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);

    println!("{:#?}", sq);
}

