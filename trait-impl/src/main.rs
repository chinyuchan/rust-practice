use std::fmt::{write, Display, Formatter};

// trait本质是一组方法原型，是实现某些目的的行为集合
// 抽象方法：没有具体实现的方法
// 具体方法：有具体实现的方法
trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

#[derive(Clone)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle({},{})", self.width, self.height)
    }
}

#[derive(Clone)]
struct Circle {
    radius: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({})", self.radius)
    }
}

// trait作为参数：表示需传入实现Geometry和Display的对象
fn print_geometry(geometry: impl Geometry + Display) {
    println!(
        "{}, area: {}, perimeter: {}",
        geometry,
        geometry.area(),
        geometry.perimeter()
    )
}

// trait约束：对泛型进行约束
fn print_geometry2<T: Geometry + Display>(geometry: &T) {
    println!(
        "{}, area: {}, perimeter: {}",
        geometry,
        geometry.area(),
        geometry.perimeter()
    )
}

// trait约束也可以用where
fn print_geometry3<T>(geometry: &T)
where
    T: Geometry + Display,
{
    println!(
        "{}, area: {}, perimeter: {}",
        geometry,
        geometry.area(),
        geometry.perimeter()
    )
}

// 返回实现trait的类型
fn return_geometry() -> impl Geometry + Display {
    Rectangle {
        width: 1.2,
        height: 3.4,
    }
}

fn main() {
    let rect = Rectangle {
        width: 1.2,
        height: 2.2,
    };
    println!("area: {}, perimeter: {}", rect.area(), rect.perimeter());
    print_geometry(rect.clone());
    print_geometry2(&rect);
    print_geometry3(&rect);

    let circle = Circle { radius: 5.0 };
    println!("area: {}, perimeter: {}", circle.area(), circle.perimeter());
    print_geometry(circle.clone());
    print_geometry2(&circle);
    print_geometry3(&circle);

    print_geometry(return_geometry());
}
