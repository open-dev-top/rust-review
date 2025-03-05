fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}", six);
    println!("none = {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 匹配是穷尽的
        None => None,
        // Some(i) => Some(i + 1),
        Some(x) => Some(x + 1),
        // error[E0308]: `match` arms have incompatible types
        // _ => (),
    }
}
