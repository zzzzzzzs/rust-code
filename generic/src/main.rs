struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32, U> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}


// 泛型
fn main() {
    let both_int = Point { x: 5, y: 10 };
    let both_int = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 4.0 };
}
