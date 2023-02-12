use std::collections::HashMap;

// HashMap 所有的键必须是相同类型，值也必须都是相同类型
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    // 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
    let mut scores: HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect();

    // TODO 遍历 HashMap 中的变量
    // 这里的 &scores 是因为，一般情况下遍历是需要对内容中的值进行修改的
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    // 插入数据
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 25);

    println!("{:?}", scores);

    //
}
