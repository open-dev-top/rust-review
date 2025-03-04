#![allow(unused)]
fn main() {
    // & 允许你使用值但不获取其所有权
    // 创建一个引用 & 的行为称为 借用（borrowing）
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("len = {len}");

    change(&mut s1);
    println!("s1 = {s1}");

    // 不能在同一时间多次将 s 作为可变变量借用
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("r1 = {r1}");
    // let r2 = &mut s;
    {
        let r2 = &mut s;
    } // r2 离开作用域失效
      // 不能在拥有不可变引用的同时拥有可变引用
    let r2 = &s;
    // println!("{}, {}", r1, r2);

    // 悬垂引用（Dangling References）
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str(", world");
}

// 不要返回局部变量的引用
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
