use std::collections::{BTreeMap, HashMap};

use macroquad::prelude::*;
mod backend;
use backend::{Business, GameData, QuantityInfo, Resource, ScaledValidRecipe, TerrainType, Tile};

use crate::backend::Building;

impl TerrainType {
    fn get_color(&self) -> Color {
        Color::from_hex(
            u32::from_str_radix(
                match self {
                    Self::Grassland => "68B75C",
                    Self::Forrest => "146842",
                    Self::Desert => "E4C670",
                    Self::Hills => "AD7135",
                    Self::Mountain => "5F4632",
                    Self::Urban => "7D847C",
                    Self::Industrial => "45818E",
                    Self::WaterShallow => "76A5AF",
                    Self::WaterDeep => "45818E",
                },
                16,
            )
            .unwrap(),
        )
    }
}

const HEX_RADIUS: f32 = 50.0;
const COS_30: f32 = 0.86602540378;
const ROAD_W: f32 = 2.0 * (1.0 - COS_30) * HEX_RADIUS;
const ICON_SIZE: f32 = 40.0;
pub const MARGIN: f32 = 10.0;
pub const TEXTURE_SIZE: f32 = 60.0;

fn get_player_color(player_id: usize) -> Color {
    match player_id {
        0 => BLUE,
        1 => GREEN,
        2 => YELLOW,
        _ => todo!(),
    }
}

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, color: Color) -> bool {
    draw_rectangle(x, y, w, h, color);
    let local_mouse_pos = Vec2::from_array(mouse_position().into()) - Vec2::new(x, y);
    local_mouse_pos.cmpgt(Vec2::ZERO).all() && local_mouse_pos.cmplt(Vec2::new(w, h)).all()
}
pub fn draw_recipes_panel(
    x: f32,
    y: f32,
    building: &mut Building,
    resource_stock: &BTreeMap<Resource, QuantityInfo>,
    textures: &HashMap<String, Texture2D>,
    editable: bool,
) -> Vec2 {
    let mut x_ = x + MARGIN;
    let mut y_ = y;

    let w = 5 as f32 * (TEXTURE_SIZE + MARGIN) + 2.0 * MARGIN + 50.0;
    let h = (TEXTURE_SIZE + MARGIN) * building.production_scale.len() as f32 + MARGIN;
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
        let click = is_mouse_button_pressed(MouseButton::Left);
        let click_up = editable && draw_button(x_, y_, 50.0, 15.0, BLACK);
        let click_down =
            editable && draw_button(x_, y_ + TEXTURE_SIZE - MARGIN - 15.0, 50.0, 15.0, BLACK);
        let requested_increment = match (
            click,
            click_up,
            click_down,
            *scale == 0,
            *scale == *max_scale,
        ) {
            (true, true, false, _, false) => 1,
            (true, false, true, false, _) => -1,
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

fn hex_idx_to_pos(x: i32, y: i32) -> Vec2 {
    Vec2::new(
        650.0 + 2.0 * HEX_RADIUS * x as f32 + HEX_RADIUS * (y % 2) as f32,
        100.0 + f32::sqrt(3.0) * HEX_RADIUS * y as f32,
    )
}

fn draw_hex(x: i32, y: i32, terrain_type: &TerrainType, border_color: Option<Color>) -> bool {
    let pos = hex_idx_to_pos(x, y);
    let mouse_position = Vec2::from_array(mouse_position().into());

    let hovering = (mouse_position - pos).length() < HEX_RADIUS
        && (0..6).any(|i| {
            let a =
                pos + HEX_RADIUS * Vec2::from_angle((i as f32 - 0.5) * std::f32::consts::PI / 3.0);
            let b =
                pos + HEX_RADIUS * Vec2::from_angle((i as f32 + 0.5) * std::f32::consts::PI / 3.0);

            let u = ((b.y - pos.y) * (mouse_position.x - pos.x)
                + (pos.x - b.x) * (mouse_position.y - pos.y))
                / ((b.y - pos.y) * (a.x - pos.x) + (pos.x - b.x) * (a.y - pos.y));
            let v = ((pos.y - a.y) * (mouse_position.x - pos.x)
                + (a.x - pos.x) * (mouse_position.y - pos.y))
                / ((b.y - pos.y) * (a.x - pos.x) + (pos.x - b.x) * (a.y - pos.y));

            u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
        });

    let color = terrain_type.get_color();
    if let Some(color) = border_color {
        draw_hexagon(pos.x, pos.y, HEX_RADIUS + 5.0, 0.0, true, WHITE, WHITE);
    }
    draw_hexagon(pos.x, pos.y, HEX_RADIUS, 0.0, true, color, color);
    if hovering {
        draw_hexagon(
            pos.x,
            pos.y,
            HEX_RADIUS,
            0.0,
            true,
            WHITE,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }
    hovering
}

fn draw_road(x: i32, y: i32, i: i32) -> bool {
    let angle = i as f32 * std::f32::consts::PI / 3.0;
    let mouse_position = Vec2::from_array(mouse_position().into());
    let pos = hex_idx_to_pos(x, y) + HEX_RADIUS * Vec2::from_angle(angle);
    let local_mouse_pos = Vec2::from_angle(-angle).rotate(mouse_position - pos);
    let hovering = local_mouse_pos
        .abs()
        .cmplt(0.5 * Vec2::new(ROAD_W, HEX_RADIUS))
        .all();
    let color = if hovering {
        Color::new(0.6, 0.6, 0.6, 1.00)
    } else {
        BLACK
    };
    draw_rectangle_ex(
        pos.x,
        pos.y,
        ROAD_W,
        HEX_RADIUS,
        DrawRectangleParams {
            rotation: i as f32 * std::f32::consts::PI / 3.0,
            offset: Vec2::new(0.5, 0.5),
            color: color,
            ..Default::default()
        },
    );
    hovering
}

#[macroquad::main("logistics-game")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    for texture_id in vec![
        "chip",
        "gold",
        "wire",
        "computer",
        "logs",
        "planks",
        "chair",
        "right_arrow",
        "money",
        "rocks",
        "sand",
        "energy",
        "raw_oil",
        "oil",
        "glass",
        "plastic",
    ] {
        textures.insert(
            texture_id.to_string(),
            load_texture(format!("assets/textures/{}.png", texture_id).as_str())
                .await
                .unwrap(),
        );
    }

    #[rustfmt::skip]
    let mut game_data=GameData{tiles: [
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Grassland, TerrainType::Grassland, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
    ].iter().map(|row| row.iter().map(|terrain_type| Tile::new(&terrain_type)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>(), businesses: vec![Business::new(),Business::new()]};

    let mut selected_hex_opt: Option<(usize, usize)> = None;
    let mut current_player_id = 0;
    loop {
        clear_background(BLACK);

        let click = is_mouse_button_pressed(MouseButton::Left);
        for (row_index, row) in game_data.tiles.iter().enumerate() {
            for (col_index, tile) in row.iter().enumerate() {
                let x = col_index as i32;
                let y = row_index as i32;
                for i in 0..3 {
                    draw_road(x, y, i);
                }
                let hovering = draw_hex(
                    x,
                    y,
                    &tile.terrain_type,
                    selected_hex_opt
                        .filter(|(selected_x, selected_y)| {
                            *selected_x == row_index && *selected_y == col_index
                        })
                        .and(Some(WHITE)),
                );
                if let Some(owner_id) = tile.owner_id {
                    let Vec2 {
                        x: marker_x,
                        y: marker_y,
                    } = hex_idx_to_pos(x, y);
                    draw_circle(marker_x, marker_y, HEX_RADIUS / 4.0, WHITE);
                    draw_circle(
                        marker_x,
                        marker_y,
                        HEX_RADIUS / 5.0,
                        get_player_color(owner_id),
                    );
                }
                if click && hovering {
                    selected_hex_opt = Some((row_index, col_index));
                }
            }
        }

        let x_ = MARGIN;
        let mut y_ = MARGIN;

        let next_turn_hovered = draw_button(x_, y_, 150.0, 30.0, RED);
        if next_turn_hovered && click {
            current_player_id += 1;
            if current_player_id == game_data.businesses.len() {
                current_player_id = 0;
            }
        }
        draw_text("End turn", x_ + MARGIN, y_ + 25.0, 28.0, WHITE);
        y_ += 50.0;
        draw_rectangle(x_, y_, 100.0, MARGIN, get_player_color(current_player_id));
        y_ += MARGIN + MARGIN;
        for (resource, quantity_info) in game_data.get_resource_stock(current_player_id) {
            draw_texture_ex(
                textures.get(&resource.get_texture_id()).unwrap(),
                x_,
                y_,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(ICON_SIZE)),
                    ..Default::default()
                },
            );

            draw_text(
                format!(
                    "{}({:+})",
                    quantity_info.quantity,
                    quantity_info.gross_in - quantity_info.gross_out,
                )
                .as_str(),
                x_ + ICON_SIZE + MARGIN,
                y_ + ICON_SIZE / 2.0,
                24.0,
                WHITE,
            );
            y_ += ICON_SIZE + MARGIN;
        }

        if is_key_pressed(KeyCode::Escape) {
            selected_hex_opt = None;
        }

        let resource_stock = game_data.get_resource_stock(current_player_id);
        let PANEL_X = 200.0;
        if let Some((tile_x, tile_y)) = selected_hex_opt {
            let tile = game_data
                .tiles
                .get_mut(tile_x)
                .unwrap()
                .get_mut(tile_y)
                .unwrap();
            draw_rectangle(PANEL_X, 0.0, 300.0, screen_height(), GRAY);
            draw_text(
                format!("{:?}", tile.terrain_type).as_str(),
                PANEL_X + MARGIN,
                MARGIN + 32.0,
                32.0,
                WHITE,
            );
            match tile.owner_id {
                Some(id) if id == current_player_id => {}
                Some(id) => {}
                None => {
                    let buy_hovered = draw_button(PANEL_X + MARGIN, 100.0, 150.0, 40.0, RED);
                    draw_text(
                        format!("Buy, ${}", tile.get_acquisition_cost()).as_str(),
                        PANEL_X + 2.0 * MARGIN,
                        120.0,
                        28.0,
                        WHITE,
                    );
                    if buy_hovered && click {
                        tile.owner_id = Some(current_player_id);
                        *game_data
                            .businesses
                            .get_mut(current_player_id)
                            .unwrap()
                            .resources
                            .get_mut(&backend::Resource::Money)
                            .unwrap() -= tile.get_acquisition_cost();
                    }
                }
            }
            let mut y_ = 200.0;
            if let Some(building) = tile.building.as_mut() {
                draw_recipes_panel(PANEL_X, y_, building, &resource_stock, &textures, true);
            } else {
                draw_text("Supported buildings:", PANEL_X + MARGIN, y_, 32.0, WHITE);
                y_ += 40.0;
                for building_type in tile.terrain_type.supported_building_types() {
                    draw_text(
                        format!("{:?}", building_type).as_str(),
                        PANEL_X + MARGIN,
                        y_,
                        24.0,
                        WHITE,
                    );
                    if tile.owner_id == Some(current_player_id) && tile.building.is_none() {
                        let hover_build = draw_button(PANEL_X + 200.0, y_ - 20.0, 100.0, 30.0, RED);
                        draw_text(
                            format!("Buy, ${}", building_type.get_construction_cost()).as_str(),
                            PANEL_X + 200.0 + MARGIN,
                            y_,
                            24.0,
                            WHITE,
                        );
                        if hover_build && click {
                            tile.building = Some(Building::new(building_type))
                        }
                    }
                    y_ += 24.0;
                }
            }
        }
        next_frame().await
    }
}
