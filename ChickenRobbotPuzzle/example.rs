mod animal_puzzle {
    // 定义一个结构体表示鸡和兔子数量的组合
    pub struct AnimalCage {
        pub chickens: i32, // 鸡的数量
        pub rabbits: i32,  // 兔子的数量
    }

    // 实现用于求解鸡兔同笼问题的相关方法
    impl AnimalCage {
        // 创建一个新的 AnimalCage 实例，包含鸡和兔子的数量
        pub fn new(chickens: i32, rabbits: i32) -> Self {
            AnimalCage { chickens, rabbits }
        }

        // 获取总的腿数，鸡有2条腿，兔子有4条腿
        pub fn total_legs(&self) -> i32 {
            self.chickens * 2 + self.rabbits * 4
        }

        // 获取总的动物数量，鸡的数量加上兔子的数量
        pub fn total_animals(&self) -> i32 {
            self.chickens + self.rabbits
        }
    }

    // 定义一个特性（Trait）用于求解鸡兔同笼问题
    pub trait PuzzleSolver {
        // 声明一个用于求解鸡兔同笼问题的函数
        fn solve(total_animals: i32, total_legs: i32) -> Option<AnimalCage>;
    }

    // 实现特性，用于求解鸡兔同笼问题
    pub struct ChickenRabbitSolver;

    impl PuzzleSolver for ChickenRabbitSolver {
        // 实现求解鸡兔同笼问题的方法
        fn solve(total_animals: i32, total_legs: i32) -> Option<AnimalCage> {
            // 遍历可能的鸡的数量，假设鸡的数量从0到总动物数
            for chickens in 0..=total_animals {
                // 兔子的数量等于总动物数减去鸡的数量
                let rabbits = total_animals - chickens;
                // 创建一个新的 AnimalCage 实例，表示当前组合的鸡和兔子的数量
                let cage = AnimalCage::new(chickens, rabbits);
                // 如果当前组合的腿数等于给定的总腿数，则返回该组合
                if cage.total_legs() == total_legs {
                    return Some(cage);
                }
            }
            // 如果遍历所有可能的组合都未找到满足条件的，返回 None
            None
        }
    }
}

fn main() {
    // 引入模块中的 ChickenRabbitSolver 和 PuzzleSolver
    use animal_puzzle::{ChickenRabbitSolver, PuzzleSolver};

    let total_animals = 10; // 假设总共有10只动物
    let total_legs = 32;    // 假设总共有32条腿

    // 使用 ChickenRabbitSolver 进行求解
    match ChickenRabbitSolver::solve(total_animals, total_legs) {
        // 如果找到符合条件的组合，打印鸡和兔子的数量
        Some(cage) => println!(
            "鸡的数量是：{}，兔子的数量是：{}",
            cage.chickens, cage.rabbits
        ),
        // 如果没有找到符合条件的组合，打印提示信息
        None => println!("没有找到符合条件的组合"),
    }
}
