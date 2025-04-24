use std::collections::HashMap;
use sdl2::{render::{Canvas, TextureCreator}, video::Window, ttf::Font};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use crate::ui::{label::UiLabel, menu::UiMenu};

pub enum UiComponents<'a> {
    Label(UiLabel),
    Menu(UiMenu<'a>),
}

pub struct UiManager<'ttf, 'rw, T> {
    components: HashMap<String, UiComponents<'rw>>,
    canvas: &'rw mut Canvas<Window>,
    font: &'rw Font<'ttf, 'rw>,
    texture_creator: &'rw TextureCreator<T>,
}

impl<'ttf, 'rw, T> UiManager<'ttf, 'rw, T> {
    pub fn new(
        canvas: &'rw mut Canvas<Window>,
        font: &'rw Font<'ttf, 'rw>,
        texture_creator: &'rw TextureCreator<T>,
    ) -> Self {
        Self {
            components: HashMap::new(),
            canvas,
            font,
            texture_creator,
        }
    }

    pub fn create_label(
        &mut self,
        key: &str,
        text: &str,
        position: Point,
        color: Color,
        use_alpha: bool,
    ) -> Result<(), String> {
        let label = UiLabel::new(text, position, color, use_alpha, self.font)?;
        self.components.insert(key.to_string(), UiComponents::Label(label));
        Ok(())
    }

    pub fn create_menu(
        &mut self, key: &str, items: Vec<&'rw str>, position: Point, item_height: i32
    )  {
        let menu = UiMenu::new(items, position, item_height);
        self.components.insert(key.to_string(), UiComponents::Menu(menu));
    }

    pub fn draw_all(&mut self) -> Result<(), String> {
        for ui in self.components.values_mut() {
            match ui {
                UiComponents::Label(l) => l.draw(self.canvas, self.texture_creator)?,
                UiComponents::Menu(m) => m.draw(self.canvas, self.font, self.texture_creator)?,
            }
        }
        Ok(())
    }

    pub fn get_label_mut(&mut self, name: &str) -> Option<&mut UiLabel> {
        match self.components.get_mut(name) {
            Some(UiComponents::Label(label)) => Some(label),
            _ => None,
        }
    }

    pub fn update_text(&mut self, key: &str, new_text: &str) {
        match self.components.get_mut(key) {
            Some(UiComponents::Label(label)) => {
                label.update_text(new_text, self.font).unwrap();
            }
            _ => (),
        }
    }

    pub fn get_menu_mut(&mut self, name: &str) -> Option<&mut UiMenu<'rw>> {
        match self.components.get_mut(name) {
            Some(UiComponents::Menu(menu)) => Some(menu),
            _ => None,
        }
    }

    pub fn end_frame(&mut self) {
        self.canvas.present();
    }

    // Optional helper to handle UI clearing from manager
    pub fn clear_background(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.set_blend_mode(sdl2::render::BlendMode::None);
        self.canvas.clear();
    }

    // Optional helper to draw background elements like rectangles (like moving boxes)
    pub fn draw_rect(&mut self, color: Color, rect: Rect) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect)?;
        Ok(())
    }
}
