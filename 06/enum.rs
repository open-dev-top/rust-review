#![allow(unused)]
fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // 直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn call(&self) {}
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    // 空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。
    // 只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空。
}
