fn main() {
    let v = [1, 2, 3, 4, 5];
    // 返回一个迭代器实例
    let mut iter = v.iter();
    // 返回迭代器中下一个元素，并用Some包装，如果到达末尾，则下一个元素为None
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // 消费器会消费（调用next方法）迭代器中元素
    let v = [1, 2, 3, 4, 5];
    // 消费器sum可对迭代器中元素执行求和
    let total: i32 = v.iter().sum();
    println!("total: {}", total);

    let v = [1, 2, 3, 4, 5];
    // 消费器any可查找迭代器中是否存在满足条件的元素
    let result1 = v.iter().any(|&x| x == 2);
    let result2 = v.iter().any(|x| *x == 2);
    println!("result1: {}, result2: {}", result1, result2);

    let v = [1, 2, 3, 4, 5];
    // 消费器collect可将迭代器转换为指定的容器，即将迭代器中的元素收集到指定的容器
    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // 迭代器适配器：会将当前迭代器转换成另一种迭代器，并支持链式调用多个迭代器适配器
    let v = [1, 2, 3, 4, 5];
    // 适配器map对迭代器中每个元素调用闭包并生成新迭代器
    let result = v.iter().map(|x| x * 10).collect::<Vec<_>>();
    println!("{:?}", result);

    let v = [1, 2, 3, 4, 5];
    // 适配器take生成一个仅迭代前n个元素的迭代器
    let result = v.iter().take(3).map(|x| x + 1).collect::<Vec<_>>();
    println!("{:?}", result);

    let v = [1, 2, 3, 4, 5];
    // 适配器filter对迭代器中每个元素执行一个返回bool的闭包并生成新的迭代器
    // 对于闭包返回bool的元素，放入新的迭代器，闭包返回false的元素，就忽略
    let result = v.iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();
    println!("{:?}", result);

    let v = [1, 2, 3, 4, 5];
    // 适配器rev生成一个反方向的迭代器
    let result = v.iter().rev().collect::<Vec<_>>();
    println!("{:?}", result);

    let v1 = [1, 3, 5];
    let v2 = [2, 4, 6, 8];
    // 适配器zip将两个迭代器压缩到一起迭代并返回一个元组，元组的第一个元素来自第一个迭代器，元组第二个元素来自第二个迭代器
    // 任意一个迭代器返回None，适配器zip就返回None
    let result = v1
        .iter()
        .zip(v2.iter())
        .map(|(x, y)| x + y)
        .collect::<Vec<_>>();
    println!("{:?}", result);
}
