// 计算阶乘，阶乘在组合数公式中是关键部分：n! = n * (n-1) * ... * 1
fn compute_factorial(n: u32) -> u32 {
    // 边界条件：0! = 1
    if n == 0 || n == 1 {
        return 1;
    }

    // 递归计算阶乘
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

// 生成所有满足 k1 + k2 + ... + km = n 的组合
fn generate_combinations_for_sum(n: u32, m: u32) -> Vec<Vec<u32>> {
    let mut combinations = Vec::new();

    // 辅助递归函数，递归构建所有满足和为n的组合
    fn helper(current: Vec<u32>, sum: u32, n: u32, m: u32, combinations: &mut Vec<Vec<u32>>) {
        // 基本条件：如果当前组合已包含m项，并且它们的和为n，则添加到结果中
        if current.len() == m as usize {
            if sum == n {
                combinations.push(current.clone());
            }
            return;
        }

        // 从0开始递归尝试每个可能的值，直到和为n
        for i in 0..=n-sum {
            let mut new_combination = current.clone();
            new_combination.push(i);
            helper(new_combination, sum + i, n, m, combinations);
        }
    }

    // 初始化递归
    helper(Vec::new(), 0, n, m, &mut combinations);
    combinations
}

// 计算组合数 C(n; k1, k2, ..., km) = n! / (k1! * k2! * ... * km!)
fn compute_combination(n: u32, combination: &Vec<u32>) -> u32 {
    // 计算n!作为分子
    let mut numerator = compute_factorial(n);
    
    // 计算每个k_i的阶乘作为分母
    let mut denominator = 1;
    for &ki in combination.iter() {
        denominator *= compute_factorial(ki);
    }

    // 返回组合数
    numerator / denominator
}

fn main() {
    // 读取用户输入，n为目标和，m为组合项数
    println!("请输入多项式的指数n：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("无法读取输入");
    let n: u32 = input.trim().parse().expect("请输入一个有效的整数");

    println!("请输入组合的项数m：");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("无法读取输入");
    let m: u32 = input.trim().parse().expect("请输入一个有效的整数");

    // 生成所有符合k1 + k2 + ... + km = n的组合
    let combinations = generate_combinations_for_sum(n, m);

    // 输出每个组合及其对应的系数
    println!("所有组合及对应的系数：");
    for combination in combinations {
        let result = compute_combination(n, &combination);  // 计算每个组合的系数
        println!("组合: {:?}, 计算结果: {}", combination, result);
    }
}
