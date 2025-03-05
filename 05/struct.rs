#![allow(unused)]
// struct 是一个块，所以不需要分号结尾
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}

fn main() {
    let cz = Student {
        name: String::from("Chang Zhao"),
        age: 18,
    };
    println!("{:#?}", cz);

    // Rust 使用类型推导风格
    let cz = build_cz(String::from("Chang Zhao"), 18);
    println!("{:#?}", cz);

    let copy_cz = Student {
        name: cz.name.clone(),
        age: cz.age,
    };
    let copy_cz = Student {
        name: cz.name,
        ..cz
    };
    println!("{:#?}", copy_cz);

    // 元组结构体
    // 每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型。
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
}

fn build_cz(name: String, age: i32) -> Student {
    Student { name, age }
}
