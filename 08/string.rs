#![allow(unused)]
fn main() {
    let mut str = String::new();
    str.push_str("This is a string."); // borowwing
    println!("{str}");

    // let data = "initial contents";
    // let s = data.to_string();
    let s = String::from("initial contents");

    // 拼接 fn add(self, s: &str) -> String {
    // 只能将 &str 和 String 相加，不能将两个 String 值相加
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                       // let s = format!("{s1}-{s2}-{s3}");
    let s = format!("{s2}-{s3}");
    println!("{s}");

    // Rust 的字符串不支持索引
    // 每个字母 2 字节
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    // let s = &hello[0..1];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
} // 当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 或字符串 slice &str 类型，而不特指其中某一个。
