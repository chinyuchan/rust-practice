fn option_add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    return if x.is_none() && y.is_none() {
        None
    } else if x.is_some() && y.is_none() {
        x
    } else if x.is_none() && y.is_some() {
        y
    } else {
        Some(x.unwrap() + y.unwrap())
    };
}

fn option_print(opt: Option<i32>) {
    match opt {
        Some(x) => println!("{}", x),
        _ => println!("None"),
    }
}

fn main() {
    let result1 = option_add(Some(10), Some(20));
    let result2 = option_add(Some(3), None);
    let result3 = option_add(None, None);

    option_print(result1);
    option_print(result2);
    option_print(result3);
}
