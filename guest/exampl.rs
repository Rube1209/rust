use std::io::{self, Write};

// 定义一个泛型 Input trait
trait Input {
    fn input() -> Self;
}

// 为 f64 类型实现 Input trait
impl Input for f64 {
    fn input() -> f64 {
        loop {
            let mut input_str = String::new();

            // 提示用户输入
            print!("请输入一个浮动值：");
            io::stdout().flush().unwrap();  // 确保提示信息立即显示

            // 读取输入
            io::stdin().read_line(&mut input_str).expect("无法读取输入");

            // 解析字符串为 f64
            match input_str.trim().parse::<f64>() {
                Ok(value) => return value,
                Err(_) => {
                    println!("无效的输入，请输入一个有效的浮动值");
                    continue;
                }
            }
        }
    }
}

// 为 i32 类型实现 Input trait
impl Input for i32 {
    fn input() -> i32 {
        loop {
            let mut input_str = String::new();
            println!("请输入一个整数：");
            io::stdout().flush().unwrap();  // 确保提示信息立即显示
            io::stdin().read_line(&mut input_str).expect("无法读取输入");

            // 解析字符串为 i32
            match input_str.trim().parse::<i32>() {
                Ok(value) => return value,
                Err(_) => {
                    println!("无效的输入，请输入一个有效的整数");
                    continue;
                }
            }
        }
    }
}

// 为 String 类型实现 Input trait
impl Input for String {
    fn input() -> String {
        let mut input_str = String::new();
        println!("请输入一个字符串：");
        io::stdout().flush().unwrap();  // 确保提示信息立即显示
        io::stdin().read_line(&mut input_str).expect("无法读取输入");

        // 去掉末尾的换行符并返回
        input_str.trim().to_string()
    }
}

fn guess() {
    let guess: i32 = 15; // 目标数字
    let mut attempts = 0; // 尝试次数

    loop {
        let num: i32 = i32::input(); // 获取用户输入

        attempts += 1;

        if num < guess {
            println!("您的输入太小了！请再试一次。");
        } else if num == guess {
            println!("恭喜！您猜对了，正确数字是：{}", guess);
            println!("您猜了 {} 次", attempts);
            break; // 猜对了，退出循环
        } else {
            println!("您的输入太大了！请再试一次。");
        }
    }
}

fn main() {
    guess();
}
