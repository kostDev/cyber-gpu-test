use sdl2::render::Canvas;
use sdl2::video::{DisplayMode, Window};
use crate::ui::rect::BoxObject;

pub trait StressScript {
    fn objects_mut(&mut self) -> &mut Vec<BoxObject>;
    fn add(&mut self, obj: BoxObject) { self.objects_mut().push(obj); }
    fn remove(&mut self) { self.objects_mut().pop(); }
    fn draw(&mut self, canvas: &mut Canvas<Window>, display_mode: &DisplayMode) -> Result<(), String> {
        self.objects_mut().iter_mut().try_for_each(|obj| {
            obj.update((display_mode.w, display_mode.h));
            obj.draw(canvas)
        })
    }
    fn count(&mut self) -> usize { self.objects_mut().len() }
}


