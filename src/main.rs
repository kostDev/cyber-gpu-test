//! Cyberico GPU Stress Test v0.1
//! Візуальний тест для GPU/VRAM на Knulli / RG35XX Plus
use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::{DisplayMode};
use std::time::Instant;
use std::fmt::Write;
use rand::Rng;

mod ui;
use ui::manager::UiManager;

const NUM_OBJECTS: usize = 12; // 30_000

struct BoxObject {
    rect: Rect,
    color: Color,
    velocity: (i32, i32),
}

impl BoxObject {
    fn update(&mut self, bounds: (i32, i32)) {
        self.rect.set_x(self.rect.x() + self.velocity.0);
        self.rect.set_y(self.rect.y() + self.velocity.1);

        if self.rect.left() < 0 || self.rect.right() > bounds.0 {
            self.velocity.0 *= -1;
        }
        if self.rect.top() < 0 || self.rect.bottom() > bounds.1 {
            self.velocity.1 *= -1;
        }
    }
}

fn random_box(display: &DisplayMode) -> BoxObject {
    let mut rng = rand::rng();
    let x = rng.random_range(0..(&display.w - 20));
    let y = rng.random_range(0..(&display.h - 20)) as i32;
    let w = rng.random_range(10..30) as u32;
    let h = rng.random_range(10..30) as u32;
    let dx = rng.random_range(-3..4);
    let dy = rng.random_range(-3..4);
    let color = Color::RGB(
        rng.random::<u8>(),
        rng.random::<u8>(),
        rng.random::<u8>(),
    );

    BoxObject {
        rect: Rect::new(x, y, w, h),
        color,
        velocity: (dx, dy),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let display_mode = video_subsystem.desktop_display_mode(0)?;
    let controller_subsystem = sdl_context.game_controller()?;
    let available = controller_subsystem.num_joysticks()?;
    let mut _controller = None;
    if available > 0 && controller_subsystem.is_game_controller(0) {
        _controller = Some(controller_subsystem.open(0)?);
    }

    let window = video_subsystem
        .window("GPU Stress Test", display_mode.w as u32, display_mode.h as u32)
        .opengl()
        .fullscreen()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 24)?;
    let texture_creator = canvas.texture_creator();
    let mut ui_manager = UiManager::new(&mut canvas, &font, &texture_creator);

    let mut event_pump = sdl_context.event_pump()?;
    let mut objects: Vec<BoxObject> = (0..NUM_OBJECTS).map(|_| random_box(&display_mode)).collect();

    let mut frame_count = 0;
    let mut last_time = Instant::now();
    let mut _fps = 0;
    let mut fps_text_buf = String::new();
    // UI
    ui_manager.create_label(
        "fps",
        "FPS: 0",
        sdl2::rect::Point::new(2, 4),
        Color::RGB(255, 255, 255),
        false,
    )?;
    ui_manager.create_menu(
        "main_menu",
        vec!["Start Stress Test", "Run Particle Mode", "Exit"],
        sdl2::rect::Point::new(210, 180),
        40,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(menu) = ui_manager.get_menu_mut("main_menu") {
                        match button {
                            Button::DPadDown => menu.move_down(),
                            Button::DPadUp => menu.move_up(),
                            // anbernic have issue with button position A -> B, Y -> X
                            Button::Start | Button::B => {
                                if menu.selected == menu.items.len() - 1 {
                                    break 'running;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        // Move all objects
        for obj in &mut objects {
            obj.update((display_mode.w, display_mode.h));
        }

        ui_manager.clear_background(Color::RGB(20, 20, 20));

        for obj in &objects {
            ui_manager.draw_rect(obj.color, obj.rect)?;
        }

        // FPS Calculation
        frame_count += 1;
        if last_time.elapsed().as_secs_f32() >= 1.0 {
            _fps = frame_count;
            frame_count = 0;
            last_time = Instant::now();
            fps_text_buf.clear();
            write!(&mut fps_text_buf, "FPS: {}", _fps)?;
            ui_manager.update_text("fps", &fps_text_buf)
        }

        ui_manager.draw_all()?;
        ui_manager.end_frame();
    }

    Ok(())
}
