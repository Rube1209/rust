use std::io;

// 阶乘函数
fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

// 生成所有满足 k1 + k2 + ... + km = n 的组合
fn generate_combinations(n: u32, m: u32) -> Vec<Vec<u32>> {
    let mut combinations = Vec::new();

    // 使用递归生成所有组合
    fn helper(curr: Vec<u32>, sum: u32, n: u32, m: u32, combinations: &mut Vec<Vec<u32>>) {
        if curr.len() == m as usize {
            if sum == n {
                combinations.push(curr);  // 如果满足条件，则保存这个组合
            }
            return;
        }

        for i in 0..=n-sum {
            let mut new_combination = curr.clone();
            new_combination.push(i);
            helper(new_combination, sum + i, n, m, combinations);  // 递归调用
        }
    }

    helper(Vec::new(), 0, n, m, &mut combinations);
    combinations
}

// 计算组合数 f(n; k1, k2, ..., km) = n! / (k1! * k2! * ... * km!)
fn compute_combination(n: u32, v: &Vec<u32>) -> u32 {
    let mut numerator = factorial(n); // 计算 n!
    let mut denominator = 1;

    for &ki in v.iter() {
        denominator *= factorial(ki); // 计算每个 ki 的阶乘并相乘
    }

    numerator / denominator // 返回结果
}

fn main() {
    // 用户输入
    println!("请输入多项式的指数n：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    let n: u32 = input.trim().parse().expect("请输入一个有效的整数");

    println!("请输入组合的项数m：");
    input.clear();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    let m: u32 = input.trim().parse().expect("请输入一个有效的整数");

    // 生成所有符合 k1 + k2 + ... + km = n 的组合
    let combinations = generate_combinations(n, m);

    // 输出每个组合及其对应的系数
    println!("所有组合及对应的系数：");
    for combination in combinations {
        let result = compute_combination(n, &combination); // 计算每个组合的系数
        println!("组合: {:?}, 计算结果: {}", combination, result);
    }
}
