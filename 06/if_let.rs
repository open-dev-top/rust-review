fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    // match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

    // if let 的 正确语法是 if let 模式 = 表达式
    // if let config_max = Some(max) {
    //     println!("The maximum is configured to be {max}");
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
