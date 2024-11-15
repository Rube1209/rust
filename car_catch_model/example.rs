struct Car {
    speed: f64,    // 当前速度
    position: f64, // 当前位置
}

impl Car {
    fn new(speed: f64, position: f64) -> Car {
        Car { speed, position }
    }
}

// 更新 A 的位置和速度，匀加速运动
fn update_position1(a: &mut Car, dt: f64, acceleration: f64) {
    a.position += a.speed * dt + 0.5 * acceleration * dt * dt; // 位移公式
    a.speed += acceleration * dt; // 更新速度
}

// 更新 B 的位置，恒定速度
fn update_position2(b: &mut Car, dt: f64) {
    b.position += b.speed * dt; // 恒定速度位移
}

// 判断 A 和 B 是否相遇，允许一个误差 delta
fn is_catch(a: &Car, b: &Car, delta: f64) -> bool {
    (a.position - b.position).abs() <= delta
}

fn main() {
    let mut car_a = Car::new(0.0, 0.0); // A 车初速度 0.0，位置 0.0
    let mut car_b = Car::new(15.0, 50.0); // B 车恒定速度 15.0，初始位置 50.0

    let delta = 0.5; // 允许的位置误差
    let mut t = 0.0; // 时间从 0 开始
    let dt = 0.1; // 时间步长
    let acceleration = 1.0; // A 的加速度，调高至 1.0

    loop {
        // 更新位置
        update_position1(&mut car_a, dt, acceleration); // A 使用时间步长 dt
        update_position2(&mut car_b, dt); // B 累加恒定速度位移

        // 打印调试信息
        println!(
            "t: {:.1}, car_a: position = {:.2}, speed = {:.2}, car_b: position = {:.2}",
            t, car_a.position, car_a.speed, car_b.position
        );

        // 判断是否相遇
        if is_catch(&car_a, &car_b, delta) {
            println!("A 和 B 在时间 {:.1} 秒相遇！", t);
            break;
        }

        t += dt; // 增加全局时间

        // 设置一个最大时间限制，防止死循环
        if t > 10000.0 {
            println!("A 和 B 在 10000 秒内没有相遇！");
            break;
        }
    }
}
