fn main() {
    // Rust 中的整型 Integer

    // 长度	有符号	无符号
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize
    println!("Size of i8: {} bytes", std::mem::size_of::<i8>());
    println!("Size of u8: {} bytes", std::mem::size_of::<u8>());
    println!("Size of i16: {} bytes", std::mem::size_of::<i16>());
    println!("Size of u16: {} bytes", std::mem::size_of::<u16>());
    println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("Size of u32: {} bytes", std::mem::size_of::<u32>());
    println!("Size of i64: {} bytes", std::mem::size_of::<i64>());
    println!("Size of u64: {} bytes", std::mem::size_of::<u64>());
    println!("Size of i128: {} bytes", std::mem::size_of::<i128>());
    println!("Size of u128: {} bytes", std::mem::size_of::<u128>());
    println!("Size of isize: {} bytes", std::mem::size_of::<isize>());
    println!("Size of usize: {} bytes", std::mem::size_of::<usize>());

    //   Rust 中的整型字面值 Integer Literal

    // 数字字面值	例子
    // Decimal (十进制)	98_222
    // Hex (十六进制)	0xff
    // Octal (八进制)	0o77
    // Binary (二进制)	0b1111_0000
    // Byte (单字节字符)(仅限于u8)	b'A'
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!(
        "Size of decimal (98_222): {} bytes",
        std::mem::size_of_val(&decimal)
    );
    println!("Size of hex (0xff): {} bytes", std::mem::size_of_val(&hex));
    println!(
        "Size of octal (0o77): {} bytes",
        std::mem::size_of_val(&octal)
    );
    println!(
        "Size of binary (0b1111_0000): {} bytes",
        std::mem::size_of_val(&binary)
    );
    println!(
        "Size of byte (b'A'): {} bytes",
        std::mem::size_of_val(&byte)
    );

    // 浮点型 Float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Size of x:f64 = {} bytes", std::mem::size_of_val(&x));
    println!("Size of y:f32 = {} bytes", std::mem::size_of_val(&y));

    // 布尔型 Boolean
    let t: bool = true;
    println!("Size of t:bool = {} bytes", std::mem::size_of_val(&t));

    // 字符类型 Character
    let c: char = 'c';
    let heart_eyed_cat = '😻';
    println!("Size of c:char = {} bytes", std::mem::size_of_val(&c));
    println!(
        "Size of heart_eyed_cat:char = {} bytes",
        std::mem::size_of_val(&heart_eyed_cat)
    );
}
