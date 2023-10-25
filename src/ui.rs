use crate::backend::{Material, OwnedBuilding, QuantityInfo, ScaledValidRecipe};
use macroquad::prelude::*;
use std::collections::{BTreeMap, HashMap};

pub const UI_BACKGROUND_COLOR: macroquad::color::Color = Color {
    r: 0.4,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};
pub const TEXTURE_SIZE: f32 = 80.0;
pub const ICON_SIZE: f32 = 50.0;
pub const MARGIN: f32 = 10.0;

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonState {
    None,
    Hover,
    Pressed,
}

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, color: Color) -> (ButtonState, Vec2) {
    draw_rectangle(x, y, w, h, color);
    let local_mouse_pos = Vec2::from_array(mouse_position().into()) - Vec2::new(x, y);
    (
        match (
            local_mouse_pos.cmpgt(Vec2::ZERO).all() && local_mouse_pos.cmplt(Vec2::new(w, h)).all(),
            is_mouse_button_pressed(MouseButton::Left),
        ) {
            (true, true) => ButtonState::Pressed,
            (true, false) => ButtonState::Hover,
            _ => ButtonState::None,
        },
        Vec2::new(w, h),
    )
}

pub fn draw_buy_ui(x: f32, y: f32, amount: i32) -> (ButtonState, Vec2) {
    let w = 350.0 + 2.0 * MARGIN;
    let h = 40.0 + 2.0 * MARGIN;
    let x_ = x + MARGIN;
    let y_ = y + MARGIN;
    draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);
    let (clicked, _button_size) = draw_button(x_, y_, w - 2.0 * MARGIN, h - 2.0 * MARGIN, RED);
    draw_text(
        format!("Buy | {}$", amount).as_str(),
        x_ + MARGIN,
        y + h / 2.0,
        32.0,
        WHITE,
    );
    (clicked, Vec2::new(w, h))
}

pub fn draw_next_turn_button(x: f32, y: f32) -> (ButtonState, Vec2) {
    let w = 150.0 + 2.0 * MARGIN;
    let h = 50.0 + 2.0 * MARGIN;
    let x_ = x + MARGIN;
    let y_ = y + MARGIN;
    draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);
    let (clicked, _button_size) = draw_button(x_, y_, w - 2.0 * MARGIN, h - 2.0 * MARGIN, RED);
    draw_text("Next turn", x_ + MARGIN, y + h / 2.0, 32.0, WHITE);
    (clicked, Vec2::new(w, h))
}

pub fn draw_message_box_ui(x: f32, y: f32, text: &str) -> Vec2 {
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
    Vec2::new(w, h)
}

pub fn draw_recipes_panel(
    x: f32,
    y: f32,
    building: &mut OwnedBuilding,
    resource_stock: &BTreeMap<Material, QuantityInfo>,
    textures: &HashMap<String, Texture2D>,
    editable: bool,
) -> Vec2 {
    let mut x_ = x + MARGIN;
    let mut y_ = y;

    let w = 5 as f32 * (TEXTURE_SIZE + MARGIN) + 2.0 * MARGIN + 50.0;
    let h = (TEXTURE_SIZE + MARGIN) * building.production_scale.len() as f32 + MARGIN;
    draw_rectangle(x, y_, w, h, UI_BACKGROUND_COLOR);
    y_ += MARGIN;
    for ScaledValidRecipe {
        valid_recipe,
        scale,
        max_scale,
    } in building.production_scale.iter_mut()
    {
        let mut texture_ids: Vec<(String, i32)> = vec![("right_arrow".to_string(), 1)];

        for (resource, quantity) in valid_recipe.get_recipe().resources.iter() {
            let index = if *quantity > 0 { texture_ids.len() } else { 0 };
            texture_ids.insert(index, (resource.get_texture_id(), quantity.abs()));
        }

        let click_up = editable && ButtonState::Pressed == draw_button(x_, y_, 50.0, 25.0, BLACK).0;
        let click_down = editable
            && ButtonState::Pressed
                == draw_button(x_, y_ + TEXTURE_SIZE - MARGIN - 25.0, 50.0, 25.0, BLACK).0;
        let requested_increment = match (click_up, click_down, *scale == 0, *scale == *max_scale) {
            (true, false, _, false) => 1,
            (false, true, false, _) => -1,
            _ => 0,
        };
        let can_increment = requested_increment != 0
            && valid_recipe
                .get_recipe()
                .resources
                .iter()
                .all(|(resource, quantity)| {
                    requested_increment * quantity > 0
                        || match resource_stock.get(resource) {
                            Some(quantity_info) => {
                                quantity_info.quantity
                                    + requested_increment * quantity
                                    + quantity_info.gross_in
                                    - quantity_info.gross_out
                                    >= 0
                            }
                            None => requested_increment * *quantity >= 0,
                        }
                });
        if can_increment {
            *scale += requested_increment;
        }
        draw_text(
            format!("{}/{}", scale, max_scale).as_str(),
            x_,
            y_ + TEXTURE_SIZE / 2.0,
            32.0,
            WHITE,
        );
        x_ += 50.0 + MARGIN;

        for (texture_id, quantity) in texture_ids {
            let texture = textures.get(&texture_id).unwrap();
            draw_texture_ex(
                &texture,
                x_,
                y_,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(TEXTURE_SIZE)),
                    ..Default::default()
                },
            );
            if quantity > 1 {
                draw_text(
                    format!("{}", quantity).as_str(),
                    x_,
                    y_ + TEXTURE_SIZE,
                    24.0,
                    WHITE,
                );
            }
            x_ += TEXTURE_SIZE + MARGIN;
        }
        x_ = x + MARGIN;
        y_ += TEXTURE_SIZE + MARGIN;
    }
    Vec2::new(w, h)
}
