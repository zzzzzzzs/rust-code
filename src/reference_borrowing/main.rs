fn main() {
    // TODO 引用 reference
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);
    println!("The length of '{}' is {}", s1, len);

    // TODO 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // TODO 数据竞争 data race
    //  多次引用同一个变量，需要在另一个变量不使用才可以
    let r1 = &mut s;
    // let r2 = &mut s; // 不可以
    println!("{}", r1);
    let r2 = &mut s;
    // println!("{}", r1); // 不可以
    println!("{}", r2);

    // TODO 悬垂引用（Dangling References）
    // let ref_to_nothing = dangle();
    let ref_to_nothing = no_dangle();
}

fn calculate_lenght(s: &String) -> usize { // & 允许使用值但不获取所有权
    s.len()
} // 这里，s 离开的作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello");
//     &s //返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。s 内存被释放。危险！

fn no_dangle() -> String { // dangle 返回一个字符串的引用
    let s = String::from("hello");
    s //返回字符串 s
} // 这里 s 的所有权被转移，因此没有错误