fn main() {
    // if
    let number = 1;
    let number = 3;
    let number = 5;
    // Rust 不支持非布尔值
    if number < 3 {
        println!("This number smaller than 3.");
    } else if number < 5 {
        println!("This number smaller than 5.");
    } else {
        // else 可以省略
        println!("This number bigger than 5.")
    }

    // if 是表达式，支持返回值
    // if 的每个分支的可能的返回值都必须是相同类型
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("number = {number}");

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // cargo fmt 会自动添加分号，这里的 break 是表达式而不是语句
            break counter * 2;
        }
    };
    println!("result = {result}");

    // loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).step_by(2).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
