fn main() {
    // 元组类型 Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    // 索引解构
    println!("First element: {}", tup.0); // 500
    println!("Second element: {}", tup.1); // 6.4
    println!("Third element: {}", tup.2); // 1

    // 模式匹配
    let (x, y, z) = tup;
    println!("x: {}", x); // 500
    println!("y: {}", y); // 6.4
    println!("z: {}", z); // 1

    // 如果表达式不返回任何其他值，则会隐式返回单元值。
    ();

    // 数组 Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let first = a[0];
    println!("first = {first}");

    // 确定元素个数不会改变时，数组会更有用
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);
}
