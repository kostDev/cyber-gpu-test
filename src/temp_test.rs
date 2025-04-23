//! Cyberico GFX Tester v0.1
//! Простий інструмент для тесту GPU на Knulli / RG35XX Plus
//! Показує FPS у верхньому правому куті

// mod temp_test;

use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn test() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let controller_subsystem = sdl_context.game_controller()?;

    let available = controller_subsystem.num_joysticks()?;
    // println!("🎮 Controllers available: {}", available);

    let mut controller = None;
    if available > 0 && controller_subsystem.is_game_controller(0) {
        controller = Some(controller_subsystem.open(0)?);
    }

    let window = video_subsystem
        .window("GFX Tester", 640, 480)
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
    let font = ttf_context.load_font("/usr/share/emulationstation/themes/es-theme-carbon/art/fonts/Cabin-Regular.ttf", 18)?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_count = 0;
    let mut last_time = Instant::now();
    let mut fps = 0;
    let mut last_input_text = Some("Просто тест тексту".to_string());

    let mut last_input_at = Instant::now();
    let timeout_duration = Duration::from_secs(60);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { button, .. } => {
                    let name = format!("Button: {:?}", button);
                    println!("{}", name);
                    last_input_text = Some(name);
                    last_input_at = Instant::now();
                }
                Event::ControllerAxisMotion { axis, value, .. } => {
                    let name = format!("Axis {:?} = {}", axis, value);
                    println!("{}", name);
                    last_input_text = Some(name);
                    last_input_at = Instant::now();
                }
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        let time = Instant::now().duration_since(last_time).as_millis() as i32;
        let y = ((time % 480) as i32).abs();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line((0, y), (640, y))?;

        frame_count += 1;
        if last_time.elapsed().as_secs_f32() >= 1.0 {
            fps = frame_count;
            frame_count = 0;
            last_time = Instant::now();
        }

        let fps_text = format!("FPS: {}", fps);
        let surface = font.render(&fps_text).blended(Color::RGB(200, 255, 200))?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;
        let target = sdl2::rect::Rect::new(640 - 120, 10, surface.width(), surface.height());
        canvas.copy(&texture, None, Some(target))?;

        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 50, 50))?;

        let x = (time % 360) as i32;
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rect(sdl2::rect::Rect::new(x, 300, 30, 30))?;

        if let Some(text) = &last_input_text {
            let input_surface = font.render(text).blended(Color::RGB(255, 255, 0))?;
            let input_texture = texture_creator.create_texture_from_surface(&input_surface)?;
            let input_target = sdl2::rect::Rect::new(20, 40, input_surface.width(), input_surface.height());
            canvas.copy(&input_texture, None, Some(input_target))?;
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));

        // println!("⏱️ Немає активності 60 секунд — вихід...");
        if last_input_at.elapsed() >= timeout_duration {
            break 'running;
        }
    }

    Ok(())
}
