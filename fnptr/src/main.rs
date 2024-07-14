fn hello() {
    println!("Hello!");
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

type MathOp = fn(i32, i32) -> i32;

// 函数作为参数
fn calc(op: MathOp, x: i32, y: i32) -> i32 {
    op(x, y)
}

// 函数作为返回值
fn math_op(op: &str) -> MathOp {
    match op {
        "add" => add,
        _ => sub,
    }
}

fn main() {
    // 必须显式指定函数类型，否则打印地址会报错
    let hello_ptr: fn() = hello;
    let add_ptr: fn(i32, i32) -> i32 = add;
    println!("hello: {:p}, add: {:p}", hello_ptr, add_ptr);
    hello_ptr();
    println!("{}", add_ptr(1, 2));

    println!("{} + {} = {}", 10, 20, calc(add, 10, 20));
    println!("{} - {} = {}", 10, 20, calc(sub, 10, 20));

    println!("{} + {} = {}", 10, 20, math_op("add")(10, 20));
    println!("{} - {} = {}", 10, 20, math_op("")(10, 20));
}
