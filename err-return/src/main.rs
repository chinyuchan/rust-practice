use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file2() -> Result<String, io::Error> {
    // ?操作符用于返回值类型为Result的函数中，可以简化match操作
    // 如果Result的值为Ok，则返回Ok中的值，并继续执行后面的代码
    // 如果Result的值为Err，则立即结束函数并将Err中的值作为函数的返回值
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用
fn read_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("content: {}", read_from_file3().unwrap());
}
