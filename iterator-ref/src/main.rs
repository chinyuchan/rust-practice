fn main() {
    // IntoIter：由into_iter方法创建，会将容器中元素的所有权转移给迭代器，显然原容器不能再使用
    let v = vec!["Rust", "C++", "C", "Java", "Golang"];
    for s in v.into_iter() {
        match s {
            "Rust" => println!("New Bee!"),
            _ => println!("{}", s),
        }
    }

    // println!("IntoIter: {:?}", v); // v moved
    println!("--------------------------------------------");
    // Iter：用iter方法创建，会将容器中元素的引用传递给迭代器，原容器可以继续使用
    let v = vec!["Rust", "C++", "C", "Java", "Golang"];
    for s in v.iter() {
        match s {
            &"Rust" => println!("New Bee!"),
            _ => println!("{}", s),
        }
    }
    println!("Iter: {:?}", v);
    println!("--------------------------------------------");
    // IterMut：由iter_mut方法创建，会将容器中元素的可变引用传递给迭代器，原容器可以继续使用
    let mut v = vec!["Rust", "C++", "C", "Java", "Golang"];
    for s in v.iter_mut() {
        match s {
            &mut "Rust" => *s = "★Rust★",
            _ => println!("{}", s),
        }
    }
    println!("{:?}", v);
}
