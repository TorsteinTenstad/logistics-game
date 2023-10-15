use macroquad::prelude::*;

use crate::backend::OwnedBuilding;

pub const UI_BACKGROUND_COLOR: macroquad::color::Color = Color {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};
pub const TEXTURE_SIZE: f32 = 100.0;
pub const MARGIN: f32 = 10.0;

pub fn draw_rectangle_with_click_detection(x: f32, y: f32, w: f32, h: f32, color: Color) -> bool {
    draw_rectangle(x, y, w, h, color);
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let local_mouse_pos = Vec2::from_array(mouse_position().into()) - Vec2::new(x, y);
    local_mouse_pos.cmpgt(Vec2::ZERO).all() && local_mouse_pos.cmplt(Vec2::new(x, y)).all()
}

pub fn draw_buy_ui(x: f32, y: f32) -> bool {
    let w = TEXTURE_SIZE + 2.0 * MARGIN;
    let h = TEXTURE_SIZE + 2.0 * MARGIN;
    let mut x_ = x + MARGIN;
    let mut y_ = y + MARGIN;
    draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);
    let clicked = draw_rectangle_with_click_detection(x_, y_, TEXTURE_SIZE, TEXTURE_SIZE, RED);
    draw_text("Buy", x_ + MARGIN, y_ + TEXTURE_SIZE / 2.0, 32.0, WHITE);
    clicked
}

pub fn draw_message_box_ui(x: f32, y: f32, text: &str) {
    let (font_size, font_scale, font_aspect) = camera_font_scale(30.0);
    let params = TextParams {
        font_size,
        font_scale,
        font_scale_aspect: font_aspect,
        ..Default::default()
    };
    let text_dimensions = measure_text(text, None, params.font_size, params.font_scale);
    let w = text_dimensions.width + 2.0 * MARGIN;
    let h = text_dimensions.height + 2.0 * MARGIN;
    draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);
    draw_text_ex(text, x + MARGIN, y + h - MARGIN, params);
}
