use macroquad::prelude::*;
mod backend;
use backend::{Business, GameData, TerrainType, Tile};

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
pub const MARGIN: f32 = 10.0;

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

fn hex_idx_to_pos(x: i32, y: i32) -> Vec2 {
    Vec2::new(
        500.0 + 2.0 * HEX_RADIUS * x as f32 + HEX_RADIUS * (y % 2) as f32,
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

    #[rustfmt::skip]
    let mut game_data=GameData{tiles: [
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Desert, TerrainType::WaterShallow, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills, TerrainType::Hills],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
        [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep, TerrainType::WaterDeep],
    ].iter().map(|row| row.iter().map(|terrain_type| Tile::new(&terrain_type)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>(), businesses: vec![Business::new()]};

    let mut selected_hex_opt: Option<(usize, usize)> = None;
    let current_player_id = 0;
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
        if is_key_pressed(KeyCode::Escape) {
            selected_hex_opt = None;
        }
        if let Some((tile_x, tile_y)) = selected_hex_opt {
            let tile = game_data
                .tiles
                .get_mut(tile_x)
                .unwrap()
                .get_mut(tile_y)
                .unwrap();
            draw_rectangle(0.0, 0.0, 400.0, screen_height(), GRAY);
            draw_text(
                format!("{:?}", tile.terrain_type).as_str(),
                MARGIN,
                MARGIN + 32.0,
                32.0,
                WHITE,
            );
            let buy_hovered = draw_button(MARGIN, 100.0, 150.0, 40.0, RED);
            draw_text(
                format!("Buy, ${}", 100.0).as_str(),
                2.0 * MARGIN,
                120.0,
                28.0,
                WHITE,
            );
            if buy_hovered && click {
                tile.owner_id = Some(current_player_id);
            }
        }
        next_frame().await
    }
}
