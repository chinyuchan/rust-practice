use std::rc::Rc;

// 一些特殊的场景，一个数据可能有多个所有者。
// Rc<T>：通过引用计数，允许一个值有多个所有者。
// 每共享一个所有权，计数就加一。当计数为0时，才析构。
fn main() {
    let x = Rc::new(100);
    println!("{:p}, count: {}", x, Rc::strong_count(&x));

    // clone之后，计数会加一
    let y = x.clone();
    println!("{:p}, count: {}", x, Rc::strong_count(&y));

    {
        let c = Rc::clone(&x);
        println!("{:p}, count: {}", x, Rc::strong_count(&c));
    }

    println!("{:p}, count: {}", x, Rc::strong_count(&x));
}
