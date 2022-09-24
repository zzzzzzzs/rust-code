fn main() {
    // 元组，一旦声明长度不能变。变量类型可以不一样
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", tup.1);
}