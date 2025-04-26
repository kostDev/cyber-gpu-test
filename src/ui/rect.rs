use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct BoxObject {
    pub rect: Rect,
    pub color: Color,
    pub velocity: (i32, i32),
}
impl BoxObject {
    pub fn new(display: (i32,i32)) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..(&display.0 - 20));
        let y = rng.random_range(0..(&display.1 - 20));
        let size = rng.random_range(12..30) as u32;
        let dx = rng.random_range(1..4) * if rng.random_bool(0.5) { 1 } else { -1 };
        let dy = rng.random_range(1..4) * if rng.random_bool(0.5) { 1 } else { -1 };
        let color = Color::RGB(
            rng.random::<u8>(),
            rng.random::<u8>(),
            rng.random::<u8>(),
        );

        BoxObject {
            rect: Rect::new(x, y, size, size),
            color,
            velocity: (dx, dy),
        }
    }
    pub fn update(&mut self, bounds: (i32, i32)) {
        self.rect.set_x(self.rect.x() + self.velocity.0);
        self.rect.set_y(self.rect.y() + self.velocity.1);

        if self.rect.left() < 0 || self.rect.right() > bounds.0 {
            self.velocity.0 *= -1;
        }
        if self.rect.top() < 0 || self.rect.bottom() > bounds.1 {
            self.velocity.1 *= -1;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect)
    }
}