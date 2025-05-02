use sdl2::render::Canvas;
use sdl2::video::{DisplayMode, Window};
use crate::ui::rect::BoxObject;

#[repr(i32)]
enum FpsLevel {
    high = 60,
    middle = 30,
    low = 20,
}

pub struct BasicStress {
    objects: Vec<BoxObject>,
    activated: bool,
    per_step: i16
}

impl BasicStress {
    pub fn new() -> Self {
        Self { objects: Vec::new(), per_step: 1000, activated: false }
    }

    pub fn update_step(&mut self, step: i16) {
        if step > 0 && step != self.per_step { self.per_step = step; }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, display_mode: &DisplayMode) -> Result<(), String> {
        if !self.activated { self.activated = true; }

        self.objects.iter_mut().try_for_each(|obj| {
            obj.update((display_mode.w, display_mode.h));
            obj.draw(canvas)
        })
    }

    pub fn finish(&mut self) {
        if self.activated {
            self.activated = false;
            self.objects.clear();
        }
    }

    pub fn watcher(&mut self, display: &DisplayMode, curr_fps: i32) {
        if self.activated && curr_fps >= FpsLevel::low as i32 {
            (0..self.per_step).for_each(|_| {
                self.objects.push(BoxObject::new((display.w, display.h)));
            });
        }
    }

    pub fn count(&mut self) -> usize {
        self.objects.len()
    }
}