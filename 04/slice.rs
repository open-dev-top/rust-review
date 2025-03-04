#![allow(unused)]
fn main() {
    let mut s = String::from("hello world");
    // 左闭右开
    let hello = &mut s[0..5];
    // cannot borrow `s` as mutable more than once at a time
    // let world = &mut s[6..11];
    // println!("{hello}{world}");

    // 可变可以给不可变调用
    let str = first_word(&s);
    println!("str = {str}");

    // 其他类型 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("{:?}", slice);
    // 数组字面量 [2, 3]
    // 数组的引用 &[2, 3]
    let result = assert_eq!(slice, &[2, 3]);
    println!("{:?}", result);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_usual(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
