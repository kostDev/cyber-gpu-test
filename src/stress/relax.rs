// use rand::Rng;
use sdl2::video::DisplayMode;
use crate::ui::rect::BoxObject;


pub struct Relax {
    // pub total: usize,
    pub objects: Vec<BoxObject>,
}

impl Relax {
    pub fn new(total: usize, display: &DisplayMode) -> Relax {
        // let mut rng = rand::rng();
        // let total_objects: usize = rng.random_range(3..16) as usize;
        let objects: Vec<BoxObject> = (0..total)
            .map(|_| BoxObject::new((display.w, display.h)))
            .collect();
        Relax {
            // total,
            objects
        }
    }

    pub fn count(&self) -> usize {
        self.objects.len()
    }
}