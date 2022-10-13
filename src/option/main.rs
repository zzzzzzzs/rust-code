// option
// Option<T> 是被包含在了 prelude 中，可以直接调用
fn main() {
    let some_num = Some(5);
    let some_char = Some('e');

    let absent_num: Option<i32> = None;

    let x = 8;
    let y: Option<i8> = Some(5);
    let sum = x + y; // 这样调用是有问题的，类型不一样
}
