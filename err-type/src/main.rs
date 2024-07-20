use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Failed to create hello.txt: {:?}", e),
    //         },
    //         _ => panic!("Failed to open hello.txt: {:?}", e),
    //     },
    // };

    // 如果Result的值是Ok，unwrap_or_else返回Ok中的值
    // 如果Result的值是Err，unwrap_or_else会执行闭包
    let file = f.unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Failed to create hello.txt: {:?}", e),
        },
        _ => panic!("Failed to open hello.txt: {:?}", e),
    });
}
