use std::io;
use num::{Zero, One};  // 引入 Zero 和 One

trait Input {
    fn input() -> Self;
}

impl Input for i32 {
    fn input() -> i32 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid integer")
    }
}

impl Input for f64 {  // 支持浮点数输入
    fn input() -> f64 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid floating point number")
    }
}

impl Input for u32 {
    fn input() -> u32 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid unsigned integer")
    }
}

// 这里为 usize 实现 Input trait
impl Input for usize {
    fn input() -> usize {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid number")
    }
}

// 计算行列式的函数
fn det<T>(matrix: Vec<Vec<T>>) -> T
where
    T: std::ops::Mul<Output = T>
    + std::ops::Sub<Output = T>
    + Zero
    + One
    + Copy
    + std::ops::Neg<Output = T>,  // 添加 Neg trait 以支持取负操作
{
    let n = matrix.len();

    // 如果矩阵是 2x2，直接计算行列式
    if n == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    // 如果矩阵是 3x3，使用行列式公式计算
    if n == 3 {
        return matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
            - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
            + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);
    }

    // 对于更大的矩阵，递归计算行列式
    let mut result = T::zero();  // 使用 T::zero() 初始化
    for i in 0..n {
        let mut submatrix = vec![];
        for j in 1..n {
            let mut row = vec![];
            for k in 0..n {
                if k != i {
                    row.push(matrix[j][k]);
                }
            }
            submatrix.push(row);
        }
        let sign = if i % 2 == 0 { T::one() } else { -T::one() };  // 符号因子
        result = result + sign * matrix[0][i] * det(submatrix);
    }
    result
}

fn main() {
    // 让用户输入矩阵的大小
    println!("Enter the size of the matrix (n x n):");
    let n: usize = usize::input();  // 使用 usize::input()

    // 创建并输入矩阵
    let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(n);  // 修改为 f64 类型的矩阵
    println!("Enter the elements row by row:");
    for i in 0..n {
        let mut row: Vec<f64> = Vec::with_capacity(n);  // 修改为 f64 类型
        for j in 0..n {
            println!("Enter element [{}][{}]:", i + 1, j + 1);
            row.push(f64::input());  // 使用改造的 input 方法读取浮点数
        }
        matrix.push(row);
    }

    // 计算并输出行列式
    let result = det(matrix);
    println!("The determinant of the matrix is: {}", result);
}
