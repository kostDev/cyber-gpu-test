// use sdl2::render::Canvas;
// use sdl2::video::{DisplayMode, Window};
use crate::stress::script::StressScript;
use crate::ui::rect::BoxObject;


pub struct BasicStress {
    objects: Vec<BoxObject>,
}

impl BasicStress {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
}

impl StressScript for BasicStress {
    fn objects_mut(&mut self) -> &mut Vec<BoxObject> {
        &mut self.objects
    }
}