//! Minimal UI library base for Cyber GPU Test
//! Provides a simple structure for rendering interactive UI components like menus
use sdl2::controller::Button;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::Window;
use crate::ui::colors::theme::{TEXT_HIGHLIGHTED, TEXT_NORMAL};
use crate::ui::enums::MenuMode;

pub struct UiMenu<'a> {
    pub items: Vec<(MenuMode, &'a str)>,
    pub item_index: usize,
    pub position: Point,
    pub spacing: i32,
    pub visible: bool,
}

impl <'a> UiMenu<'a> {
    pub fn new(items: Vec<(MenuMode, &'a str)>, position: Point, spacing: i32) -> Self {
        Self {
            items,
            item_index: 0,
            position,
            spacing,
            visible: true,
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
        if self.items.is_empty() || !self.visible {
            return Ok(());
        }

        for (i, (_, label)) in self.items.iter().enumerate() {
            let color = if i == self.item_index {
                TEXT_HIGHLIGHTED
            } else {
                TEXT_NORMAL
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

    pub fn handle_menu_input(&mut self, button: Button) -> (bool, MenuMode) {
        match button {
            Button::DPadDown => self.move_down(),
            Button::DPadUp => self.move_up(),
            //                                  selected status, MenuMode
            Button::Start | Button::B => {
                // self.visible = false;
                return (true, self.items[self.item_index].0)
            },
            _ => {}
        }
        (false, self.items[self.item_index].0) // selected status, MenuMode
    }
}

// TODO: Later expand this with Button, Checkbox, ScrollList, etc.
// TODO: Goal: composable UI that can be customized or reused across future Cyber tools.
