#![allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数（associated functions）
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        // self	方法的参数，表示当前实例
        // Self	类型别名，表示当前 impl 块对应的类型
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rect1 = {rect1}");
    // dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用）
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area() // 方法语法（method syntax）
                     // 自动引用和解引用（automatic referencing and dereferencing）
                     // 等价于 &rect1.area()
                     // 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是
                     // 仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
