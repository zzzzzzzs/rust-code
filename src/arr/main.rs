// 如果想让你的数据存放在stack（栈）上而不是 heap（堆）上，或者想保证有固定数量的元素，这时使用数组更有好处
// Vector和数组类似，它由标准库提供
// Vector的长度可以改变
// 如果你不确定应该用数组还是Vector，那么估计你应该用Vector.


// 数组是Stack上分配的单个块的内存。
// 可以使用索引来访问数组的元素。
// 如果访问的索引超出了数组的范围:
// 编译会通过
// 运行会报错（runtime时会panic)
// Rust不会允许其继续访问相应地址的内存

fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);

    let a = [3; 5];
    // 相当于
    // let a = [3, 3, 3, 3, 3];

    let first = months[0];
}