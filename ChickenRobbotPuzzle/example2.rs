struct Chicken {
    num: i32,
    total_foot: i32,
}

struct Rabbit {
    num: i32,
    total_foot: i32,
}

// 条件判断函数：检查鸡和兔子数量和脚数是否匹配
fn conditions(head_num: i32, foot_num: i32, chickens: &Chicken, rabbits: &Rabbit) -> bool {
    let total_heads = chickens.num + rabbits.num;  // 鸡兔头数之和
    let total_feet = chickens.total_foot + rabbits.total_foot;  // 鸡兔脚数之和

    total_heads == head_num && total_feet == foot_num  // 判断是否符合给定条件
}

fn solve(head_num: i32, foot_num: i32) {
    // 使用两个 for 循环，遍历鸡和兔子的数量
    for chicken_count in 0..=head_num { // 假设最多不超过头数
        let rabbit_count = head_num - chicken_count; // 兔子的数量 = 总头数 - 鸡的数量

        // 计算当前鸡兔的脚数
        let chickens = Chicken {
            num: chicken_count,
            total_foot: chicken_count * 2, // 每只鸡 2 条腿
        };

        let rabbits = Rabbit {
            num: rabbit_count,
            total_foot: rabbit_count * 4, // 每只兔子 4 条腿
        };

        // 判断条件
        if conditions(head_num, foot_num, &chickens, &rabbits) {
            println!("鸡的数量: {}, 兔的数量: {}", chickens.num, rabbits.num);
            return; // 找到解后退出循环
        }
    }

    // 如果没有找到符合条件的解
    println!("没有符合条件的解");
}

fn main() {
    let head_num = 35; // 总头数
    let foot_num = 94; // 总脚数
    solve(head_num, foot_num);  // 调用 solve 函数进行条件验证
}
