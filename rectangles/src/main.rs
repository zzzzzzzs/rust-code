#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// TODO 求长方形面积
fn main() {
    let width1 = 30;
    let height1 = 50;

    // TODO 普通方法
    println!("{}", area1(width1, height1));

    // TODO 元组方法
    let rect = (30, 50);
    println!("{}", area2(rect));

    // TODO 结构体方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rect1);
    println!("{}", area3(&rect1));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}


fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}