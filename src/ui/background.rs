use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{ Canvas };
use sdl2::video::Window;

pub struct UiBackground {
    pub position: Point,
    pub size: (u32, u32),
    pub color: Color,
    pub visible: bool,
    rect: Rect,
}

impl UiBackground {
    pub fn new(position: Point, size: (u32, u32), color: Color, visible: bool) -> UiBackground {
        UiBackground {
            position,
            size,
            color,
            visible,
            rect: Rect::new(position.x, position.y, size.0, size.1)
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.visible {
            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            canvas.set_draw_color(self.color);
            canvas.fill_rect(self.rect)?;
            canvas.set_blend_mode(sdl2::render::BlendMode::None);
        }
        Ok(())
    }

    // pub fn set_size(&mut self, width: u32, height: u32) {
    //     self.size = (width, height);
    //     self.rect.set_width(width);
    //     self.rect.set_height(height);
    // }
    //
    // pub fn set_position(&mut self, x: i32, y: i32) {
    //     self.position = Point::new(x, y);
    //     self.rect.set_x(x);
    //     self.rect.set_y(y);
    // }
}