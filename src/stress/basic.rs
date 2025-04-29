use sdl2::video::DisplayMode;
// use sdl2::render::Canvas;
// use sdl2::video::{DisplayMode, Window};
use crate::stress::script::StressScript;
use crate::ui::rect::BoxObject;

#[repr(i32)]
enum FpsLevel {
    high = 60,
    middle = 30,
    low = 20,
}

pub struct BasicStress {
    objects: Vec<BoxObject>,
    generate_per_step: i16
}

impl BasicStress {
    pub fn new() -> Self {
        Self { objects: Vec::new(), generate_per_step: 200 }
    }

    pub fn update_generate_per_step(&mut self, step: i16) {
        if step > 0 && step != self.generate_per_step { self.generate_per_step = step; }
    }

    pub fn add_objects(&mut self, display: &DisplayMode, curr_fps: i32) {
        if curr_fps >= FpsLevel::low as i32 {
            (0..self.generate_per_step).for_each(|_| {
                self.add(BoxObject::new((display.w, display.h)));
            });
        }
    }

    pub fn finish(&mut self) {
        self.objects.clear();
    }
}

impl StressScript for BasicStress {
    fn objects_mut(&mut self) -> &mut Vec<BoxObject> {
        &mut self.objects
    }
}