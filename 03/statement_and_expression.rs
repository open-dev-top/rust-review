// 函数定义也是语句
fn main() {
    // 语句不返回值
    // let x = (let y = 6);

    // 表达式会计算出一个值
    let y = {
        let x = 3;
        x + 1 // 如果在表达式的结尾加上分号，它就变成了语句。
    };
    println!("y = {y}");
}
