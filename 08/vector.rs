#![allow(unused)]

use std::vec;
fn main() {
    // 注意这里我们增加了一个类型注解。因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素。
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let v = vec![1, 2];
    let third: Option<&i32> = v.get(2);
    if let Some(third) = third {
        println!("The third element is {third}");
    } else {
        println!("No third element found.");
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
} // 类似于任何其他的 struct，vector 在其离开作用域时会被释放
