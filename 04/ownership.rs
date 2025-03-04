// Rust 中的每一个值都有一个 所有者（owner）。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
#![allow(unused)]
fn main() {
    // scope
    {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
        let s = String::from("hello");
        // 使用 s
    } // Rust 在结尾的 } 处自动调用 drop

    // String
    let mut s = String::from("hello");

    // 说明 String 存储在栈上
    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{s}"); // 将打印 `hello, world!`

    // move
    // 在栈上的会复制而不是移动
    let x = 5;
    let y = x;
    println!("x={x}");
    println!("y={y}");

    // 在堆上的会移动
    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();
    println!("{s1}, world!");
    println!("{s2}, world!");

    // move with function
    let s = String::from("owner");
    take_ownership(s);
    // println!("s = {s}");

    let i = 0;
    give_ownership(i);
    println!("i = {i}");

    let s1 = give_out_ownership(); // gives_ownership 将返回值
                                   // 转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中，
                                       // 它也将返回值移给 s3
}

fn take_ownership(some_string: String) {
    println!("some_string = {some_string}");
}

fn give_ownership(some_integer: i32) {
    println!("some_integer = {some_integer}");
}

fn give_out_ownership() -> String {
    // give_out_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string
                // 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
