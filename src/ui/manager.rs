use std::collections::HashMap;
use sdl2::{render::{Canvas, TextureCreator}, video::Window, ttf::Font };
use crate::ui::{label::UiLabel, menu::UiMenu};

pub enum UiComponents<'a> {
    Label(UiLabel),
    Menu(UiMenu<'a>),
}

pub struct UiManager<'ttf, 'rw, T> {
    pub components: HashMap<String, UiComponents<'rw>>,
    pub canvas: &'rw mut Canvas<Window>,
    pub font: &'rw Font<'ttf, 'rw>,
    pub texture_creator: &'rw TextureCreator<T>,
}

impl <'ttf, 'rw, T> UiManager<'ttf, 'rw, T> {
    pub fn new(
        canvas: &'rw mut Canvas<Window>,
        font: &'rw Font,
        texture_creator: &'rw TextureCreator<T>,
    ) -> Self {
        Self {
            components: HashMap::new(),
            canvas,
            font,
            texture_creator
        }
    }

    pub fn add_label(&mut self, name: &str, label: UiLabel) {
        self.components.insert(name.to_string(), UiComponents::Label(label));
    }

    pub fn add_menu(&mut self, name: &str, menu: UiMenu) {
        self.components.insert(name.to_string(), UiComponents::Menu(menu));
    }

    pub fn draw(&mut self) -> Result<(), String> {
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

    pub fn get_menu_mut(&mut self, name: &str) -> Option<&mut UiMenu> {
        match self.components.get_mut(name) {
            Some(UiComponents::Menu(menu)) => Some(menu),
            _ => None,
        }
    }

}