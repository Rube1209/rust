// 定义一个名为 Person 的结构体，表示一个人
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    // 创建一个新的 Person 实例
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    // 枚举可能的年龄解，找到符合条件的 (m, g) 年龄对
    fn solve_age_problem(&self, other: &Person) {
        // 枚举所有可能的年龄解
        for m in 1..=48 {  // 妈妈的年龄 m
            for g in 1..=48 {  // 女孩的年龄 g
                // 检查条件：m + g = 48 和 m = 4g - 2
                if m + g == 48 && m == 4 * g - 2 {
                    println!("找到解：今年妈妈的年龄是：{}，女孩的年龄是：{}", m, g);

                    // 倒推年份：检查几年之前妈妈的年龄是女孩的 5 倍
                    let mut years_ago = 0;
                    while m - years_ago != 5 * (g - years_ago) {
                        years_ago += 1;
                    }

                    // 输出结果：返回找到的年份
                    println!("几年前，妈妈的年龄是女孩的 5 倍：{} 年前", years_ago);
                    return;  // 找到解后结束函数
                }
            }
        }
        println!("没有找到符合条件的解");
    }
}

fn main() {
    // 创建两个 Person 实例：妈妈和女孩
    let mother = Person::new("妈妈".to_string(), 34);
    let girl = Person::new("女孩".to_string(), 14);

    // 调用 solve_age_problem 方法来解题
    girl.solve_age_problem(&mother);
}
