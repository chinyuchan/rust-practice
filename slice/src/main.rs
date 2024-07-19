fn print_str(s: &str) {
    println!("slice: {}, len: {}", s, s.len())
}

fn print_vec(v: &[i32]) {
    println!("slice: {:?}, len: {:?}", v, v.len())
}

fn main() {
    let s = String::from("Hello,Rust!");
    println!("{}", &s[0..5]);
    println!("{}", &s[..5]);
    println!("{}", &s[6..s.len()]);
    println!("{}", &s[6..]);
    println!("{}", &s[0..s.len()]);
    println!("{}", &s[..]);

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", &v[0..3]);
    println!("{:?}", &v[..3]);
    println!("{:?}", &v[4..]);
    println!("{:?}", &v[4..v.len()]);
    println!("{:?}", &v[0..v.len()]);
    println!("{:?}", &v[..]);

    print_str(&s);
    print_str(&s[..5]);
    print_vec(&v);
    print_vec(&v[..5]);

    // 可变切片
    let mut v = vec![1, 2];
    let vs = &mut v;
    vs.push(3);
    println!("{:?}", vs);
}
