use crate::garden::vegetables::Asparagus;

/// 声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden;声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
/// 内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
/// 在文件 src/garden.rs
/// 在文件 src/garden/mod.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
