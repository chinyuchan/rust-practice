use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let file = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open hello.txt: {:?}", e)
        }
    };

    // 如果Result的值是Ok，unwrap返回Ok中的值
    // 如果Result的值是Err，unwrap会自动panic并输出错误信息
    let file = File::open("hello.txt").unwrap();

    // 兼顾unwrap的功能，并可以自定义错误信息
    let file = File::open("hello.txt").expect("Failed to open hello.txt");
}
