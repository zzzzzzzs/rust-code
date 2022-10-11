fn main() {

    // TODO 移动 move
    let x = 5;
    let y = x;
    // TODO 栈上分配的变量可以进行拷贝
    println!("{}", x);


    let s1 = String::from("hello");
    let s2 = s1;

    // TODO 这里 s1 所有权改变到了 s2 上，那么 s1 就无效了
    // println!("{}, world!", s1);

    // TODO 克隆 clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 堆上的数据被复制了

    println!("s1 = {}, s2 = {}", s1, s2);

    // TODO 所有权与函数
    let s = String::from("hello"); // s 是分配在堆上的
    takes_ownership(s); // s 的值移动到函数里
    // println!("{}", s); // 再次使用 s 的值就无效了,所以这里会出问题

    let x = 5; // x 是分配在栈上的
    makes_copy(x); // x 应该是移动到函数里的，但 i32 是可以 copy 的
    println!("{}", x); // 这里还是可以使用 x 的

    // TODO 返回值与作用域
    let s1 = gives_ownership(); // gives_ownership 将返回值转移给 s1
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中，s2 的所有权被移动了，不可以再次调用 s2，然后将返回值移动给 s3
    println!("{}, {}", s1, s3);
}


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移除作用域并调用 drop 方法
// 占用的内存被释放

fn makes_copy(some_int: i32) { // some_int 进入作用域, 将传入的 i32 变量值拷贝了一份
    println!("{}", some_int);
} // 这里，将 some_int 移除作用域，由于是栈上分配的，移除的是拷贝的值


fn gives_ownership() -> String { // gives_ownership 会将返回值移动给调用它的函数
    let some_string = String::from("yours");
    some_string // 返回 some_string，并移除给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移除给调用的函数
}