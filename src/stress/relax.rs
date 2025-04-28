// use rand::Rng;
use sdl2::video::DisplayMode;
use sdl2::render::{Canvas};
use sdl2::video::Window;
use crate::ui::rect::BoxObject;

pub struct Relax {
    pub objects: Vec<BoxObject>,
}

impl Relax {
    pub fn new(total: usize, display: &DisplayMode) -> Relax {
        // let mut rng = rand::rng();
        // let total_objects: usize = rng.random_range(3..16) as usize;
        let objects: Vec<BoxObject> = (0..total)
            .map(|_| BoxObject::new((display.w, display.h)))
            .collect();
        Relax { objects }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, display_mode: &DisplayMode) -> Result<(), String> {
        self.objects.iter_mut().try_for_each(|obj| {
            obj.update((display_mode.w, display_mode.h));
            obj.draw(canvas)
        })
    }

    pub fn count(&self) -> usize {
        self.objects.len()
    }
}