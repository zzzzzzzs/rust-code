// match 模式匹配
// 必须穷举所有可能
// _ 通配符：替代其余没有列举出的值,且必须放在最后
// if let
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // 按顺序匹配
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c));

    let v:u8 = 8;
    match v {
        1 => println!("one"),
        _ => println!("other"),
    }

    // if let 放弃穷举了所有的可能
    if let 3 = v {
        println!("three");
    } else {
        println!("other");
    }

}



