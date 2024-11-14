mod kindergarten {
    // 定义一个结构体表示糖果分发情况
    pub struct CandyDistribution {
        chocolates: i32,     // 巧克力总数
        milk_candies: i32,   // 奶糖总数
        fruit_candies: i32,  // 水果糖总数
        children: i32,       // 小朋友数量
    }

    // 实现糖果分发相关的方法
    impl CandyDistribution {
        pub fn new(chocolates: i32, milk_candies: i32, fruit_candies: i32, children: i32) -> Self {
            CandyDistribution {
                chocolates,
                milk_candies,
                fruit_candies,
                children,
            }
        }

        // 验证当前数量是否满足条件
        pub fn is_valid_distribution(&self) -> bool {
            // 分给每个小朋友的糖果数量
            let total_chocolates_distributed = 2 * self.children;
            let total_milk_candies_distributed = 7 * self.children;
            let total_fruit_candies_distributed = 8 * self.children;

            // 计算剩余的糖果数量
            if self.chocolates < total_chocolates_distributed
                || self.milk_candies < total_milk_candies_distributed
                || self.fruit_candies < total_fruit_candies_distributed
            {
                return false;
            }

            let remaining_chocolates = self.chocolates - total_chocolates_distributed;
            let remaining_milk_candies = self.milk_candies - total_milk_candies_distributed;
            let remaining_fruit_candies = self.fruit_candies - total_fruit_candies_distributed;

            // 条件1：水果糖剩下15块
            // 条件2：剩下的巧克力是奶糖的3倍
            remaining_fruit_candies == 15 && remaining_chocolates == 3 * remaining_milk_candies
        }
    }

    // 计算小朋友数量
    pub fn calculate_children() -> Option<i32> {
        // 我们假设每种糖果的总数是相同的
        let mut total_candies = 1;
        loop {
            let children = total_candies / 17; // 每个小朋友需要 2 + 7 + 8 = 17 颗糖果
            let distribution = CandyDistribution::new(total_candies, total_candies, total_candies, children);

            if distribution.is_valid_distribution() {
                return Some(children);
            }

            total_candies = total_candies.saturating_add(1);
        }
    }
}

fn main() {
    match kindergarten::calculate_children() {
        Some(children) => println!("小朋友的数量是：{}", children),
        None => println!("没有找到符合条件的小朋友数量"),
    }
}
