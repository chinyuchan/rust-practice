use std::cmp::Ordering;

struct Person {
    id: i32,
    name: String,
    height: i32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.height.cmp(&other.height))
    }
}

fn main() {
    let person1 = Person {
        id: 1,
        name: "zhangsan".to_string(),
        height: 170,
    };
    let person2 = Person {
        id: 2,
        name: "lisi".to_string(),
        height: 175,
    };
    let person3 = Person {
        id: 3,
        name: "wangwu".to_string(),
        height: 168,
    };
    println!("person1 < person2: {}", person1 < person2);
    println!("person1 < person3: {}", person1 < person3);
}
