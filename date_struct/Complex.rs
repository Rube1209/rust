use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

// 实现 fmt::Display 特性，方便打印 Complex 结构体
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// 实现加法
impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

// 实现减法
impl Sub for &Complex {
    type Output = Complex;

    fn sub(self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

// 实现乘法
impl Mul for &Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

// 实现除法
impl Div for &Complex {
    type Output = Complex;

    fn div(self, other: &Complex) -> Complex {
        let denominator = other.real * other.real + other.imag * other.imag;
        Complex {
            real: (self.real * other.real + self.imag * other.imag) / denominator,
            imag: (self.imag * other.real - self.real * other.imag) / denominator,
        }
    }
}

fn main() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);

    println!("Sum: {}", &c1 + &c2);
    println!("Difference: {}", &c1 - &c2);
    println!("Product: {}", &c1 * &c2);
    println!("Quotient: {}", &c1 / &c2);

    println!("c1: {}", c1); // 使用引用后，c1 和 c2 可以继续使用
    println!("c2: {}", c2);
}
