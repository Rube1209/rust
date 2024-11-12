use std::io;

trait Input {
    fn input() -> Self;
}

// 实现 Input trait 的各种类型

// 8位无符号整数
impl Input for u8 {
    fn input() -> u8 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u8>().unwrap()
    }
}

// 32位整数
impl Input for i32 {
    fn input() -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    }
}

// 64位整数
impl Input for i64 {
    fn input() -> i64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i64>().unwrap()
    }
}

// 64位浮点数
impl Input for f64 {
    fn input() -> f64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<f64>().unwrap()
    }
}

// 32位浮点数
impl Input for f32 {
    fn input() -> f32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<f32>().unwrap()
    }
}

// 布尔值
impl Input for bool {
    fn input() -> bool {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_lowercase();
        match trimmed.as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => panic!("输入无效，必须是 true/false 或 1/0"),
        }
    }
}

// 字符类型
impl Input for char {
    fn input() -> char {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().chars().next().unwrap_or_else(|| panic!("输入无效，必须输入一个字符"))
    }
}

// 字符串
impl Input for String {
    fn input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

macro_rules! Input {
    ($type:ty) => {
        <$type>::input()
    };
}

fn main() {
    // 示例：读取各种类型的输入

    // 8位无符号整数
    println!("请输入一个 u8 数字:");
    let u8_value: u8 = Input!(u8);
    println!("你输入的 u8 数字是: {}", u8_value);

    // 32位整数
    println!("请输入一个 i32 数字:");
    let i32_value: i32 = Input!(i32);
    println!("你输入的 i32 数字是: {}", i32_value);

    // 64位整数
    println!("请输入一个 i64 数字:");
    let i64_value: i64 = Input!(i64);
    println!("你输入的 i64 数字是: {}", i64_value);

    // 64位浮点数
    println!("请输入一个 f64 数字:");
    let f64_value: f64 = Input!(f64);
    println!("你输入的 f64 数字是: {}", f64_value);

    // 32位浮点数
    println!("请输入一个 f32 数字:");
    let f32_value: f32 = Input!(f32);
    println!("你输入的 f32 数字是: {}", f32_value);

    // 布尔值
    println!("请输入一个布尔值 (true/false 或 1/0):");
    let bool_value: bool = Input!(bool);
    println!("你输入的布尔值是: {}", bool_value);

    // 字符类型
    println!("请输入一个字符:");
    let char_value: char = Input!(char);
    println!("你输入的字符是: {}", char_value);

    // 字符串
    println!("请输入一个字符串:");
    let string_value: String = Input!(String);
    println!("你输入的字符串是: {}", string_value);
}
