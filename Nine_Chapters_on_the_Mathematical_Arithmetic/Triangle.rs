use std::f64;

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    // 新建一个 Triangle 实例
    fn new(a: f64, b: f64, c: f64) -> Self {
        Triangle { a, b, c }
    }

    // 计算三角形面积
    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0; // 半周长
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt() // 使用海伦公式计算面积
    }
}

fn main() {
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    println!("三角形的面积是: {}", triangle.area());
}
