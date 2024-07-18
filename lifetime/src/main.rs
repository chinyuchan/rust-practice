fn boo(s: &str) -> &str {
    s
}

fn long_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "Rust".to_string();
    let s2 = "C".to_string();
    println!("{}", long_str(&s1, &s2));
}
