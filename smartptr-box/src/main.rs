// Box<T>：独占所有权，数据放在堆上，指向堆数据的指针放到栈上。
// 离开作用域时，会销毁内部指针，并释放堆数据。

#[derive(Debug)]
struct Custom {
    data: String,
}
impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping Custom with data: {}", self.data);
    }
}

fn main() {
    let x: Box<i32> = Box::new(100);
    // x moved
    let y = x;
    // println!("{}", x);
    println!("{}, {:p}", y, y);

    let x: Box<i32> = Box::new(200);
    // 解引用，i32实现了Copy，会复制，不会move
    let y = *x;
    println!("{}", x);
    println!("{}", y);

    let c1 = Custom {
        data: "Rust".to_string(),
    };
    println!("{}", c1.data);
}
