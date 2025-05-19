use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;
use crate::ui::colors::theme::MaybeColor;

pub struct UiLabel {
    pub text: String,
    pub position: Point,
    pub color: Color,
    pub surface: Option<Surface<'static>>,
    pub visible: bool,
}

impl UiLabel {
    pub fn new(text: &str, position: Point, color: Color, font: &Font) -> Result<Self, String> {
        let surface = font.render(text).blended(color).map_err(|e| e.to_string())?;
        Ok(Self {
            text: text.to_string(),
            position,
            color,
            surface: Some(surface),
            visible: true,
        })
    }

    pub fn update_text(&mut self, new_text: &str, font: &Font, color: MaybeColor) -> Result<(), String> {
        let new_color = color.unwrap_or(self.color);

        if self.text != new_text ||  self.color != new_color {
            self.text = new_text.to_string();
            self.color = new_color;
            self.surface = Some(font.render(&self.text).blended(self.color).map_err(|e| e.to_string())?);
        }

        Ok(())
    }

    pub fn draw<T>(&self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<T>) -> Result<(), String> {
        if let Some(surface) = &self.surface {
            let texture = texture_creator.create_texture_from_surface(surface).map_err(|e| e.to_string())?;
            let target = Rect::new(self.position.x, self.position.y, surface.width(), surface.height());
            canvas.copy(&texture, None, Some(target))?;
        }
        Ok(())
    }
}