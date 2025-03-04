fn main() {
    let x = 1;
    println!("x = {x}");

    {
        let x = x + 1;
        println!("x = {x}");
    }

    let x = x + 2;
    println!("x = {x}");

    // 隐藏和 mut 的区别：
    // 1.对变量重新赋值时，使用 let，新变量仍然不可变。
    // 2.使用 let，相当于创建了新变量，变量类型可以改变。
    let spaces = "   ";
    println!("spaces = {spaces}");

    let spaces = spaces.len();
    println!("spaces = {spaces}");
}
