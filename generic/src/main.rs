struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn width(&self) -> &T {
        &self.width
    }

    fn height(&self) -> &T {
        &self.height
    }
}

// 只有i32类型的Rectangle才有这个方法
impl Rectangle<i32> {
    fn area(&self) -> i32 {
        self.width * self.width
    }
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle2<T, U> {
    fn width(&self) -> &T {
        &self.width
    }

    fn height(&self) -> &U {
        return &self.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };
    println!(
        "width: {}, height: {}, area: {}",
        rect1.width(),
        rect1.height(),
        rect1.area()
    );

    let rect2 = Rectangle2 {
        width: 3,
        height: 4.5,
    }; // T:i32, U:f64
    println!("width: {}, height: {}", rect2.width(), rect2.height());

    let rect2 = Rectangle2 {
        width: 3.21,
        height: 4.56,
    }; // T:f64, U:f64
    println!("width: {}, height: {}", rect2.width(), rect2.height());
}
