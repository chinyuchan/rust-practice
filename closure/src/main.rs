fn main() {
    let a = 10;
    // 闭包和函数的最大区别在于：闭包可以捕获其被定义作用域中的变量
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one(a));
}
