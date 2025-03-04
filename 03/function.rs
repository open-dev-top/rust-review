fn main() {
    my_function();
    another_function(6);
    print_labeled_measurement(5, 'h');

    let x = 1;
    let x = plus_one(x);
    println!("x = {x}");

    hello();
}

// snake case 规范风格：所有字母都是小写并使用下划线分隔单词
// Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。
fn my_function() {
    println!("my_function is called.")
}

// Rust 不支持函数重载
// 在函数签名中，必须 声明每个参数的类型。
fn another_function(x: i32) {
    println!("another_function({}) is called.", x);
}

// 定义多个参数时，使用逗号分隔。
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 带返回值的函数
fn plus_one(x: i32) -> i32 {
    x + 1
}

/* println!() 既可以作为表达式，也可以作为语句
不是所有宏的返回值都是 ()（unit 类型）
Rust 的宏（macro_rules! 或 proc_macro）可以返回任何类型
具体取决于宏的定义和展开后的代码
*/
fn hello() -> () {
    println!("hello")
}
