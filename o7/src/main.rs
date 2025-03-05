//! 模块系统（the module system）
//!
//! 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
//! Crates ：一个模块的树形结构，它形成了库或二进制项目。
//! 模块（Modules）和 use：允许你控制作用域和路径的私有性。
//！ 路径（path）：一个命名例如结构体、函数或模块等项的方式。

/// crate 是 Rust 在编译时最小的代码单位。
///
/// crate 有两种形式：二进制项和库。
/// 二进制项 可以被编译为可执行程序，比如一个命令行程序或者一个 web server。它们必须有一个 main 函数来定义当程序被执行的时候所需要做的事情。目前我们所创建的 crate 都是二进制项。
/// 库 并没有 main 函数，它们也不会编译为可执行程序，它们提供一些诸如函数之类的东西，使其他项目也能使用这些东西。比如 第二章 的 rand crate 就提供了生成随机数的东西。大多数时间 Rustaceans 说的 crate 指的都是库，这与其他编程语言中 library 概念一致。

/// 包（package）是提供一系列功能的一个或者多个 crate。
/// 一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。
/// 包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。

// src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
// Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。
fn main() {
    println!("Hello, world!");
}
