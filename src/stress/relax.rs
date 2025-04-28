// use rand::Rng;
use sdl2::video::DisplayMode;
// use sdl2::render::{Canvas};
// use sdl2::video::Window;
// use crate::stress::basic::BasicStress;
use crate::ui::rect::BoxObject;
use crate::stress::script::StressScript;

pub struct Relax {
    objects: Vec<BoxObject>,
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
}

impl StressScript for Relax {
    fn objects_mut(&mut self) -> &mut Vec<BoxObject> {
        &mut self.objects
    }
}