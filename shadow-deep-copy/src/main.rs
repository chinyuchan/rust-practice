type Record = (i32, f64, bool);

#[derive(Debug)]
struct Student {
    no: i32,
    score: f64,
    pass: bool,
}
#[derive(Debug)]
enum Class {
    GradeOne(i32),
    GradeTwo(i32),
}

fn main() {
    // 元组中每个元素实现了Copy则该元组就可以浅复制
    let r1: Record = (1001, 75.8, true);
    let r2 = r1;
    println!("{:?}, {:?}", r1, r2);

    // 对于结构体和枚举，即使所有字段都实现了Copy，也不支持浅复制
    let s1 = Student {
        no: 1002,
        score: 59.0,
        pass: false,
    };
    let s2 = s1; // s1 moved
                 // println!("{:?}", s1);
    println!("{:?}", s2);

    let class1 = Class::GradeOne(1);
    let class2 = class1; // class1 moved
    // println!("{:?}", class1);
    println!("{:?}", class2);
}
