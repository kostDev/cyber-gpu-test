//! Cyberico GPU Stress Test v0.1
//! Візуальний тест для GPU/VRAM на Knulli / RG35XX Plus
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use rand::{Rng};

const NUM_OBJECTS: usize = 600;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

struct BoxObject {
    rect: Rect,
    color: Color,
    velocity: (i32, i32),
}

fn random_box() -> BoxObject {
    let mut rng = rand::rng();
    let x = rng.random_range(0..(SCREEN_WIDTH - 20)) as i32;
    let y = rng.random_range(0..(SCREEN_HEIGHT - 20)) as i32;
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

    let window = video_subsystem
        .window("GPU Stress Test", SCREEN_WIDTH, SCREEN_HEIGHT)
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

    let mut event_pump = sdl_context.event_pump()?;
    let mut objects: Vec<BoxObject> = (0..NUM_OBJECTS).map(|_| random_box()).collect();

    let mut frame_count = 0;
    let mut last_time = Instant::now();
    let mut fps = 0;

    let last_input_at = Instant::now();
    let timeout_duration = Duration::from_secs(60);

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Move all objects
        for obj in &mut objects {
            obj.rect.set_x(obj.rect.x() + obj.velocity.0);
            obj.rect.set_y(obj.rect.y() + obj.velocity.1);

            if obj.rect.left() < 0 || obj.rect.right() as u32 > SCREEN_WIDTH {
                obj.velocity.0 *= -1;
            }
            if obj.rect.top() < 0 || obj.rect.bottom() as u32 > SCREEN_HEIGHT {
                obj.velocity.1 *= -1;
            }
        }

        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        for obj in &objects {
            canvas.set_draw_color(obj.color);
            canvas.fill_rect(obj.rect)?;
        }

        // FPS Calculation
        frame_count += 1;
        if last_time.elapsed().as_secs_f32() >= 1.0 {
            fps = frame_count;
            frame_count = 0;
            last_time = Instant::now();
        }
        let fps_text = format!("FPS: {}", fps);
        let surface = font.render(&fps_text).blended(Color::RGB(255, 255, 255))?;
        // Background cover for FPS text
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 148));
        // (fps_text.len() * 20) as u32
        canvas.fill_rect(Rect::new(0, 0, surface.width() + 8, surface.height() + 8))?;
        canvas.set_blend_mode(sdl2::render::BlendMode::None);
        // FPS text render
        let texture = texture_creator.create_texture_from_surface(&surface)?;
        let target = sdl2::rect::Rect::new(2, 4, surface.width(), surface.height());
        canvas.copy(&texture, None, Some(target))?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));

        if last_input_at.elapsed() >= timeout_duration {
            println!("⏱️ Немає активності 60 секунд — вихід...");
            break 'running;
        }
    }

    Ok(())
}
