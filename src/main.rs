//! Cyber GPU Stress Test v0.1
//! Візуальний тест для GPU/VRAM на Knulli / RG35XX Plus
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Instant;
use std::fmt::Write;

mod ui;
use ui::{manager::UiManager, enums::MenuMode, rect::BoxObject};

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

    let total_objects: usize = 12; // 30_000
    let mut event_pump = sdl_context.event_pump()?;
    let mut objects: Vec<BoxObject> = (0..total_objects)
        .map(|_| BoxObject::new((display_mode.w, display_mode.h)))
        .collect();

    let mut frame_count = 0;
    let mut last_time = Instant::now();
    let mut _fps = 0;
    let mut fps_text_buf = String::new();

    let items = vec![
        (MenuMode::Basic, "GPU Stress Test"),
        (MenuMode::FillScreen, "Run Boxes Mode"),
        (MenuMode::Particle, "Run Particle Mode"),
        (MenuMode::Exit, "Exit"),
    ];

    // UI
    ui_manager.create_menu(
        "main_menu",
        items,
        sdl2::rect::Point::new(210, 180),
        40,
    );
    ui_manager.create_label(
        "fps",
        "FPS: 0",
        sdl2::rect::Point::new(2, 4),
        Color::RGB(255, 255, 255),
        false,
    )?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { button, .. } => {
                    if let Some((mode, true)) = ui_manager
                        .get_menu_mut("main_menu")
                        .map(|m| m.handle_menu_input(button))
                    {
                        match mode {
                            MenuMode::Basic => { /* ... */ }
                            MenuMode::FillScreen => { /* ... */ }
                            MenuMode::Particle => { /* ... */ }
                            MenuMode::Exit => break 'running,
                        }
                    }
                }
                _ => {}
            }
        }

        ui_manager.clear_background(Color::RGB(20, 20, 20));
        // Move all objects & update
        objects.iter_mut().try_for_each(|obj| {
            obj.update((display_mode.w, display_mode.h));
            ui_manager.draw_rect(obj.color, obj.rect)
        })?;

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
