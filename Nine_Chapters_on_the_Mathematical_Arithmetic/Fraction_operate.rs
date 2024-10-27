// 定义一个分数结构体
struct Fraction {
    a: i32, // 分子
    b: i32, // 分母
}

impl Fraction {
    // 构造函数，创建一个新的分数
    fn new(a: i32, b: i32) -> Self {
        let mut fraction = Fraction { a, b };
        fraction.simplify(); // 在构造时直接进行约分
        fraction
    }

    // 求最大公约数的辅助函数，用于约分，确保 a >= b
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        // 确保 a >= b
        if b > a {
            std::mem::swap(&mut a, &mut b);
        }

        // 使用循环计算 GCD
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }

    // 约分函数，将分数约分为最简形式
    fn simplify(&mut self) {
        let gcd = Fraction::gcd(self.a, self.b);
        self.a /= gcd;
        self.b /= gcd;
    }

    // 分数加法，返回一个新的分数
    fn add(&self, other: &Fraction) -> Fraction {
        let numerator = self.a * other.b + other.a * self.b; // 计算新分子
        let denominator = self.b * other.b; // 计算新分母
        Fraction::new(numerator, denominator) // 自动约分
    }

    // 分数减法，返回一个新的分数
    fn sub(&self, other: &Fraction) -> Fraction {
        let numerator = self.a * other.b - other.a * self.b; // 计算新分子
        let denominator = self.b * other.b; // 计算新分母
        Fraction::new(numerator, denominator) // 自动约分
    }

    // 分数乘法，返回一个新的分数
    fn mul(&self, other: &Fraction) -> Fraction {
        let numerator = self.a * other.a; // 计算新分子
        let denominator = self.b * other.b; // 计算新分母
        Fraction::new(numerator, denominator) // 自动约分
    }

    // 打印分数
    fn display(&self) {
        println!("{}/{}", self.a, self.b);
    }
}

fn main() {
    let fraction1 = Fraction::new(3, 4);
    let fraction2 = Fraction::new(1, 2);

    // 测试加法
    let result_add = fraction1.add(&fraction2);
    print!("加法结果: ");
    result_add.display(); // 输出结果：5/4

    // 测试减法
    let result_sub = fraction1.sub(&fraction2);
    print!("减法结果: ");
    result_sub.display(); // 输出结果：1/4

    // 测试乘法
    let result_mul = fraction1.mul(&fraction2);
    print!("乘法结果: ");
    result_mul.display(); // 输出结果：3/8
}
