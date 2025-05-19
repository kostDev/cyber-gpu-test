use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::ui::colors::theme::get_random_rgb_color;

pub struct RectObject {
    pub rect: Rect,
    pub color: Color,
    pub velocity: (i32, i32),
}
impl RectObject {
    pub fn new(display: (i32,i32)) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..(&display.0 - 28));
        let y = rng.random_range(0..(&display.1 - 28));
        let size = rng.random_range(12..28) as u32;
        let dx = rng.random_range(1..3) * if rng.random_bool(0.5) { 1 } else { -1 };
        let dy = rng.random_range(1..3) * if rng.random_bool(0.5) { 1 } else { -1 };

        RectObject {
            rect: Rect::new(x, y, size, size),
            color: get_random_rgb_color(rng),
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

    pub fn change_color(&mut self) {
        self.color = get_random_rgb_color(rand::rng());
    }
}