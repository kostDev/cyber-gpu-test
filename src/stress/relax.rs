use sdl2::render::Canvas;
use sdl2::video::{DisplayMode, Window};
use crate::ui::rect::RectObject;

pub struct Relax {
    objects: Vec<RectObject>,
    activated: bool,
}
impl Relax {
    pub fn new(total: usize, display: &DisplayMode) -> Relax {
        // let mut rng = rand::rng();
        // let total_objects: usize = rng.random_range(3..16) as usize;
        let objects: Vec<RectObject> = (0..total)
            .map(|_| RectObject::new((display.w, display.h)))
            .collect();
        Relax { objects, activated: false }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, display_mode: &DisplayMode) -> Result<(), String> {
        if !self.activated { self.activated = true; }

        self.objects.iter_mut().try_for_each(|obj| {
            obj.update((display_mode.w, display_mode.h));
            obj.draw(canvas)
        })
    }

    pub fn count(&mut self) -> usize {
        self.objects.len()
    }
}
