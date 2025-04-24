//! Minimal UI library base for Cyber GPU Test
//! Provides a simple structure for rendering interactive UI components like menus

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::Window;

pub struct UiMenu<'a> {
    pub items: Vec<&'a str>,
    pub selected: usize,
    pub position: Point,
    pub spacing: i32,
}

impl<'a> UiMenu<'a> {
    pub fn new(items: Vec<&'a str>, position: Point, spacing: i32) -> Self {
        Self {
            items,
            selected: 0,
            position,
            spacing,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.items.len() {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<Window>, font: &Font, texture_creator: &TextureCreator<T>) -> Result<(), String> {
        if self.items.is_empty() {
            return Ok(());
        }

        for (i, item) in self.items.iter().enumerate() {
            let color = if i == self.selected {
                Color::RGB(255, 255, 0)
            } else {
                Color::RGB(200, 200, 200)
            };

            let surface = font.render(item).blended(color).map_err(|e| e.to_string())?;
            let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
            let target = Rect::new(
                self.position.x,
                self.position.y + (i as i32 * self.spacing),
                surface.width(),
                surface.height(),
            );
            canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            canvas.copy(&texture, None, Some(target))?;
            canvas.set_blend_mode(sdl2::render::BlendMode::None);
        }

        Ok(())
    }
}

// TODO: Later expand this with Button, Checkbox, ScrollList, etc.
// TODO: Goal: composable UI that can be customized or reused across future Cyber tools.
