fn calculate_pi(n: i32) {
    let mut s: f64;

    // 转换 n 为 f64 以进行浮点运算
    let n_f64 = n as f64;

    // 计算 s = 0.5 * n * sin(2π / n)
    s = 0.5 * n_f64 * (2.0 * std::f64::consts::PI / n_f64).sin();

    println!("n = {}, Approximated π = {}", n, s);
}

fn main() {
    // 测试不同的 n 值
    for &n in &[6, 12, 100, 1000, 10000] {
        calculate_pi(n);
    }
}
