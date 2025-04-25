//! Minimal UI library base for Cyber GPU Test
//! Provides a simple structure for rendering interactive UI components like menus
use sdl2::controller::Button;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::Window;
use crate::ui::enums::MenuMode;

pub struct UiMenu<'a> {
    pub items: Vec<(MenuMode, &'a str)>,
    pub item_index: usize,
    pub selected: bool,
    pub position: Point,
    pub spacing: i32,
}

impl<'a> UiMenu<'a> {
    pub fn new(items: Vec<(MenuMode, &'a str)>, position: Point, spacing: i32) -> Self {
        Self {
            items,
            item_index: 0,
            selected: false,
            position,
            spacing,
        }
    }

    pub fn move_up(&mut self) {
        if self.item_index > 0 {
            self.item_index -= 1;
        } else {
            self.item_index = self.items.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.item_index + 1 < self.items.len() {
            self.item_index += 1;
        } else {
            self.item_index = 0;
        }
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<Window>, font: &Font, texture_creator: &TextureCreator<T>) -> Result<(), String> {
        if self.items.is_empty() {
            return Ok(());
        }

        for (i, (_, label)) in self.items.iter().enumerate() {
            let color = if i == self.item_index {
                Color::RGB(255, 255, 0)
            } else {
                Color::RGB(200, 200, 200)
            };

            let surface = font.render(label).blended(color).map_err(|e| e.to_string())?;
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

    pub fn handle_menu_input(&mut self, button: Button) -> (MenuMode, bool) {
        match button {
            Button::DPadDown => self.move_down(),
            Button::DPadUp => self.move_up(),
            Button::Start | Button::B => return (self.items[self.item_index].0, true),
            _ => {}
        }
        (self.items[self.item_index].0, false)
    }
}

// TODO: Later expand this with Button, Checkbox, ScrollList, etc.
// TODO: Goal: composable UI that can be customized or reused across future Cyber tools.
