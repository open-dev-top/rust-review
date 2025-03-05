// 使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。
use o7_restaurant::hosting;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    hosting::add_to_waitlist();
}

// 使用 use 语句将两个具有相同名称的项带入作用域
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

// 使用 as 关键字提供新的名称
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

// 嵌套路径来消除大量的 use 行
// use std::{cmp::Ordering, io};
