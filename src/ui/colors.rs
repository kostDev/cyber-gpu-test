pub mod theme {
    use sdl2::pixels::Color;

    pub type MaybeColor = Option<Color>;
    pub const TEXT_NORMAL: Color = Color::RGB(200, 200, 200);
    pub const TEXT_HIGHLIGHTED: Color = Color::RGB(255, 255, 0);
    pub const BACKGROUND: Color = Color::RGB(0, 0, 0);
    pub const FPS_LABEL: Color = Color::RGB(100, 255, 100);
    pub const BUTTON_LABEL: Color = Color::RGB(255, 100, 100);

    pub const TEMP_COLOR_DANGER: Color = Color::RGB(246, 4, 125);
    pub const TEMP_COLOR_HOT: Color = Color::RGB(246, 32, 0);
    pub const TEMP_COLOR_MID: Color = Color::RGB(246, 188, 0); // yellow
    pub const TEMP_COLOR_GOOD: Color = Color::RGB(0, 240, 0); // green

    // Визначаємо колір в залежності від температури
    pub fn get_temp_color(temp: f32) -> Color {
        if temp >= 85.0 { TEMP_COLOR_DANGER }
        else if temp >= 70.0 { TEMP_COLOR_HOT }
        else if temp >= 60.0 { TEMP_COLOR_MID }
        else if temp >= 50.0 { TEMP_COLOR_MID }
        else { TEXT_NORMAL }
    }

}