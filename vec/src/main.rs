#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


// vec 元素在内存中是连续的，在往 vue 中 push 元素的时候有可能会重新分配内存。
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("{:?}", v1);

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);

    println!("{}", v2[2]);

    let i = &v2[0];
    println!("hhh {}", *i);

    match v2.get(2) {
        Some(third) => println!("{}", third),
        None => println!("None"),
    }

    for i in &mut v1 {
        *i += 50;
    }
    for i in v1 {
        println!("{}", i);
    }

    // TODO 例子
    //  vector只能存放相同类型的数据，但是Enum的存在提供了一种在vector中存放不同类型数据的方式
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("row is {:?}", row);
}