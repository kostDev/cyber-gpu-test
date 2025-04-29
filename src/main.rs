//! Cyber GPU Stress Test v0.1
//! Візуальний тест для GPU/VRAM на RG35XX Plus
use std::time::{Instant, Duration};
use std::fmt::Write;
use std::fs;
use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::ttf::Font;

mod ui;
mod stress;

use ui::{
    menu::UiMenu,
    label::UiLabel,
    enums::MenuMode,
    colors::theme::{get_temp_color, BACKGROUND, TEXT_NORMAL, OBJECTS_LABEL}
};
use stress::relax::Relax;
use crate::stress::basic::BasicStress;
use crate::stress::script::StressScript;

pub struct Fonts<'a> {
    pub xs: Font<'a, 'static>,
    pub sm: Font<'a, 'static>,
    pub md: Font<'a, 'static>,
    pub lg: Font<'a, 'static>,
}

// TEMP °C: CPU, GPU, DDR
// CPU: /sys/class/thermal/thermal_zone{0}/temp
// GPU: /sys/class/thermal/thermal_zone{1}/temp
// DDR: /sys/class/thermal/thermal_zone{3}/temp
fn read_temperature(zone: u8) -> Option<f32> {
    let path = format!("/sys/class/thermal/thermal_zone{}/temp", zone);
    let content = fs::read_to_string(path).ok()?;
    let temp_milli_celsius: i32 = content.trim().parse().ok()?;
    Some(temp_milli_celsius as f32 / 1000.0)
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
        // .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let fonts = Fonts {
        xs: ttf_context.load_font("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 12)?,
        sm: ttf_context.load_font("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 16)?,
        md: ttf_context.load_font("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 24)?,
        lg: ttf_context.load_font("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 32)?,
    };
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    // init modes
    let mut relax_obj = Relax::new(42, &display_mode);
    let mut basic_obj = BasicStress::new();
    let relax_mode: &mut dyn StressScript = &mut relax_obj;
    let basic_mode: &mut dyn StressScript = &mut basic_obj;

    let mut frame_count = 0;
    let mut total_rect_obj = 0;
    let mut last_time = Instant::now();
    let mut fps = 0;
    let mut fps_text_buf = String::new();

    let items = vec![
        (MenuMode::Basic, "GPU Stress Test"),
        (MenuMode::FillScreen, "Run Boxes Mode"),
        (MenuMode::Particle, "Run Particle Mode"),
        (MenuMode::Relax, "Run Relax Mode"),
        (MenuMode::Exit, "Exit"),
    ];

    // UI
    let mut menu = UiMenu::new(
        items,
        sdl2::rect::Point::new(195, 182),
        40,
    );
    let mut label_fps = UiLabel::new(
        "FPS: 0",
        sdl2::rect::Point::new(2, 4),
        TEXT_NORMAL,
        false,
        &fonts.md,
    )?;
    // temperature
    let mut temperature_cpu = UiLabel::new(
        "CPU: * °C",
        sdl2::rect::Point::new(2, 42),
        TEXT_NORMAL,
        false,
        &fonts.xs,
    )?;
    let mut temperature_gpu = UiLabel::new(
        "GPU: * °C",
        sdl2::rect::Point::new(2, 60),
        TEXT_NORMAL,
        false,
        &fonts.xs
    )?;
    let mut temperature_ddr = UiLabel::new(
        "DDR: * °C",
        sdl2::rect::Point::new(2, 78),
        TEXT_NORMAL,
        false,
        &fonts.xs
    )?;
    let mut label_rect_objs = UiLabel::new(
        "ROB: 0",
        sdl2::rect::Point::new(2, 96),
        OBJECTS_LABEL,
        false,
        &fonts.xs,
    )?;

    'running: loop {
        let frame_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { button, .. } => {
                    match button {
                        Button::Guide => {
                            menu.show();
                            if total_rect_obj > 0 {
                                basic_obj.finish();
                                total_rect_obj = 0;
                            }
                        }
                        Button::DPadDown => menu.move_down(),
                        Button::DPadUp => menu.move_up(),
                        Button::Start | Button::B => {
                           menu.hide();
                        }
                        _ => {}
                    }

                }
                _ => {}
            }
        }

        canvas.set_blend_mode(sdl2::render::BlendMode::None);
        canvas.clear();
        canvas.set_draw_color(BACKGROUND);
        canvas.fill_rect(None)?;



        // FPS Calculation
        frame_count += 1;
        // per 1 second
        if last_time.elapsed().as_secs_f32() >= 1.0 {
            fps = frame_count;
            frame_count = 0;
            last_time = Instant::now();

            let new_fps = format!("FPS: {}", fps);

            if new_fps != fps_text_buf {
                fps_text_buf.clear();
                fps_text_buf.push_str(&new_fps);
                label_fps.update_text(&fps_text_buf, &fonts.md, None)?;
            }

            let temp_cpu = read_temperature(0).unwrap();
            let temp_gpu = read_temperature(1).unwrap();
            let temp_ddr = read_temperature(3).unwrap();

            temperature_cpu.update_text(
                &format!("CPU: {:.1} °C" , temp_cpu),
                &fonts.xs,
                Some(get_temp_color(temp_cpu))
            )?;
            temperature_gpu.update_text(
                &format!("GPU: {:.1} °C" , temp_gpu),
                &fonts.xs,
                Some(get_temp_color(temp_gpu))
            )?;
            temperature_ddr.update_text(
                &format!("DDR: {:.1} °C" , temp_ddr),
                &fonts.xs,
                Some(get_temp_color(temp_ddr))
            )?;

            label_rect_objs.update_text(
                &format!("ROB: {}" , total_rect_obj),
                &fonts.xs,
                None
            )?;
        }
        // render mode
        if let Some(selected) = menu.selected_item() {
            match selected {
                MenuMode::Basic => {
                    basic_mode.draw(&mut canvas, &display_mode)?;
                    total_rect_obj = basic_obj.count();
                }
                MenuMode::FillScreen => { /* ... */ }
                MenuMode::Particle => { /* ... */ }
                MenuMode::Relax => {
                    relax_mode.draw(&mut canvas, &display_mode)?;
                    total_rect_obj = relax_mode.count();
                }
                MenuMode::Exit => { break 'running },
            }
        }
        // ui render
        menu.draw(&mut canvas, &texture_creator,&fonts.lg)?;
        label_fps.draw(&mut canvas, &texture_creator)?;
        temperature_cpu.draw(&mut canvas, &texture_creator)?;
        temperature_gpu.draw(&mut canvas, &texture_creator)?;
        temperature_ddr.draw(&mut canvas,  &texture_creator)?;
        label_rect_objs.draw(&mut canvas,  &texture_creator)?;

        canvas.present();

        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(1) {
            std::thread::sleep(Duration::from_millis(1) - frame_time);
        }
    }

    Ok(())
}
