// 定义一个结构体来表示分数
struct Fraction {
    p: i32, // 分子
    q: i32, // 分母
}

impl Fraction {
    // 构造函数，创建一个新的分数，并自动约分
    fn new(p: i32, q: i32) -> Self {
        let gcd = Fraction::gcd(p, q);
        Fraction {
            p: p / gcd,
            q: q / gcd,
        }
    }

    // 计算最大公约数 (GCD) 的方法
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Fraction::gcd(b, a % b)
        }
    }

    // 约分函数，直接修改分数到最简形式
    fn simplify(&mut self) {
        let gcd = Fraction::gcd(self.p, self.q);
        self.p /= gcd;
        self.q /= gcd;
    }

    // 打印分数
    fn display(&self) {
        println!("{}/{}", self.p, self.q);
    }
}

fn main() {
    let mut fraction = Fraction::new(8, 12); // 创建一个8/12的分数，会自动约分为2/3
    fraction.display(); // 输出：2/3

    let mut fraction2 = Fraction { p: 10, q: 20 };
    fraction2.simplify(); // 约分为1/2
    fraction2.display(); // 输出：1/2
}
