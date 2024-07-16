// 引用：是一种语法
// 借用：对引用行为的描述

// 先获得所有权，再返回所有权，这种写法太繁琐
fn sum_vec1(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    let s1 = v1.iter().sum::<i32>();
    let s2 = v2.iter().sum::<i32>();
    (v1, v2, s1 + s2)
}

// 通过引用传参，即借用所有权，更简洁
fn sum_vec2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let s1 = v1.iter().sum::<i32>();
    let s2 = v2.iter().sum::<i32>();
    s1 + s2
}

// 可变引用
fn push_vec(v: &mut Vec<i32>, value: i32) {
    v.push(value)
}

fn main() {
    let v1 = vec![1, 3, 5];
    let v2 = vec![2, 4, 6];
    let (v1, v2, sum) = sum_vec1(v1, v2);
    println!("{:?}, {:?}, {}", v1, v2, sum);

    let sum = sum_vec2(&v1, &v2);
    println!("{:?}, {:?}, {}", v1, v2, sum);

    let mut v1 = vec![1, 3, 5];
    push_vec(&mut v1, 100);
    println!("{:?}", v1);
}
