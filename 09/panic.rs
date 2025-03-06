/// 在实践中有两种方法造成 panic：执行会造成代码 panic 的操作（比如访问超过数组结尾的内容）或者显式调用 panic! 宏。
/// 通常情况下这些 panic 会打印出一个错误信息，展开并清理栈数据，然后退出。
///
/// 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接 终止（abort），这会不清理数据就退出程序。
fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    v[99];
}

// RUST_BACKTRACE=1 cargo run --bin panic
// 为了获取带有这些信息的 backtrace，必须启用 debug 标识。当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用，就像这里一样。
