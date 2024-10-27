use piston_window::*;

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    fn new(x: f64, y: f64, r: f64) -> Self {
        Self { x, y, r }
    }

    fn draw(&self, window: &mut PistonWindow) {
        let mut events = window.events;
        while let Some(e) = events.next(window) {
            window.draw_2d(&e, |c, g, _| {
                clear([1.0; 4], g);
                ellipse([0.5, 0.5, 0.5, 1.0],  // 圆的颜色
                        [self.x - self.r, self.y - self.r, self.r * 2.0, self.r * 2.0],  // 圆的边界
                        c.transform, g);
            });
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Circle Example", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let circle = Circle::new(200.0, 200.0, 50.0);
    circle.draw(&mut window);
}
