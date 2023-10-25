use std::{
    collections::{BTreeMap, HashMap},
    default,
};

use macroquad::prelude::*;
mod backend;
use backend::{
    BuildingType, Business, GameData, QuantityInfo, Resource, Road, ScaledValidRecipe, TerrainType,
    Tile,
};

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
                    Self::Industrial => "CCEBC5",
                    Self::WaterShallow => "76A5AF",
                    Self::WaterDeep => "45818E",
                },
                16,
            )
            .unwrap(),
        )
    }
}

impl BuildingType {
    fn get_texture_id(&self) -> &str {
        match self {
            Self::LocalMarket => "market",
            Self::EnergyMarket => "market",
            Self::Sawmill => "saw",
            Self::FurnitureFactory => "factory",
            Self::WoodWorkingMarket => "market",
            Self::ComputerFactory => "factory",
            Self::SandPlant => "digger",
            Self::Mine => "minecart",
            Self::MetalRefinery => "factory",
            Self::GlassFactory => "factory",
            Self::OilRig => "oil_rig",
            Self::OilRefinery => "oil_refinery",
            Self::PlasticFactory => "factory",
            Self::OilEnergyPlant => "fossil_plant",
            Self::TreeFarm => "forrest_farm",
            Self::SolarEnergyFarm => "solar_plant",
            Self::WindEnergyFarm => "wind_plant",
        }
    }
}

const HEX_RADIUS: f32 = 80.0;
const COS_30: f32 = 0.86602540378;
const ROAD_W: f32 = 2.0 * (1.0 - COS_30) * HEX_RADIUS;
const ICON_SIZE: f32 = 40.0;
const SELECTED_BORDER_W: f32 = 4.0;
pub const MARGIN: f32 = 10.0;
pub const TEXTURE_SIZE: f32 = 60.0;

fn get_player_color(player_id: usize) -> Color {
    Color::from_hex(
        u32::from_str_radix(
            match player_id {
                0 => "B3CDE3",
                1 => "FBB4AE",
                2 => "CCEBC5",
                _ => "",
            },
            16,
        )
        .unwrap(),
    )
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

fn hex_idx_to_pos(x: usize, y: usize) -> Vec2 {
    Vec2::new(
        680.0 + 2.0 * HEX_RADIUS * x as f32 + HEX_RADIUS * (y % 2) as f32,
        100.0 + f32::sqrt(3.0) * HEX_RADIUS * y as f32,
    )
}

fn draw_hex(x: usize, y: usize, terrain_type: &TerrainType, border_color: Option<Color>) -> bool {
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

fn draw_road(
    tile_a_idx: (usize, usize),
    tile_b_idx: (usize, usize),
    selected: bool,
    owner_color: Option<Color>,
) -> bool {
    let tile_a_pos = hex_idx_to_pos(tile_a_idx.0, tile_a_idx.1);
    let tile_b_pos = hex_idx_to_pos(tile_b_idx.0, tile_b_idx.1);
    let angle = (tile_b_pos - tile_a_pos).angle_between(Vec2::new(1.0, 0.0));
    let mouse_position = Vec2::from_array(mouse_position().into());
    let pos = (tile_a_pos + tile_b_pos) / 2.0;
    let local_mouse_pos = Vec2::from_angle(-angle).rotate(mouse_position - pos);
    let hovering = local_mouse_pos
        .abs()
        .cmplt(0.5 * Vec2::new(HEX_RADIUS, ROAD_W))
        .all();
    if selected {
        draw_rectangle_ex(
            pos.x,
            pos.y,
            HEX_RADIUS + 2.0 * SELECTED_BORDER_W,
            ROAD_W + 2.0 * SELECTED_BORDER_W,
            DrawRectangleParams {
                rotation: -angle,
                offset: Vec2::new(0.5, 0.5),
                color: WHITE,
                ..Default::default()
            },
        );
    }
    if let Some(color) = owner_color {
        draw_rectangle_ex(
            pos.x,
            pos.y,
            HEX_RADIUS,
            ROAD_W,
            DrawRectangleParams {
                rotation: -angle,
                offset: Vec2::new(0.5, 0.5),
                color,
                ..Default::default()
            },
        );
    }
    if hovering || selected {
        draw_rectangle_ex(
            pos.x,
            pos.y,
            HEX_RADIUS,
            ROAD_W,
            DrawRectangleParams {
                rotation: -angle,
                offset: Vec2::new(0.5, 0.5),
                color: Color::new(1.0, 1.0, 1.0, 0.5),
                ..Default::default()
            },
        );
    }
    hovering
}

#[derive(PartialEq, Eq)]
enum SelectedAsset {
    TileIds(usize, usize),
    RoadIds(usize),
    None,
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
        "saw",
        "factory",
        "market",
        "factory",
        "digger",
        "minecart",
        "factory",
        "factory",
        "oil_rig",
        "oil_refinery",
        "factory",
        "fossil_plant",
        "forrest_farm",
        "solar_plant",
        "wind_plant",
    ] {
        textures.insert(
            texture_id.to_string(),
            load_texture(format!("assets/textures/{}.png", texture_id).as_str())
                .await
                .unwrap(),
        );
    }

    #[rustfmt::skip]
    let mut game_data= GameData::new(
        [
            [TerrainType::Hills, TerrainType::Hills, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain],
            [TerrainType::Desert, TerrainType::Hills, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Hills],
            [TerrainType::Desert, TerrainType::Hills, TerrainType::Mountain, TerrainType::Mountain, TerrainType::Hills, TerrainType::Industrial, TerrainType::Industrial],
            [TerrainType::Forrest, TerrainType::Forrest, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial, TerrainType::Industrial],
            [TerrainType::Mountain, TerrainType::Mountain, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::Urban, TerrainType::Industrial],
            [TerrainType::Mountain, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Forrest, TerrainType::Urban, TerrainType::Urban, TerrainType::WaterShallow],
            [TerrainType::Mountain, TerrainType::Industrial, TerrainType::Industrial, TerrainType::Forrest, TerrainType::Forrest, TerrainType::WaterShallow, TerrainType::WaterShallow],
            ]
            .iter()
            .map(|row| {
                row.iter()
                    .map(|terrain_type| Tile::new(&terrain_type))
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>(),
            2,
        );

    let mut selected_opt: SelectedAsset = SelectedAsset::None;
    let mut current_player_id = 0;
    loop {
        clear_background(BLACK);

        let click = is_mouse_button_pressed(MouseButton::Left);
        for (x, row) in game_data.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let hovering = draw_hex(
                    x,
                    y,
                    &tile.terrain_type,
                    match selected_opt {
                        SelectedAsset::TileIds(selected_x, selected_y)
                            if selected_x == x && selected_y == y =>
                        {
                            Some(WHITE)
                        }
                        _ => None,
                    },
                );
                if let Some(owner_id) = tile.owner_id {
                    let Vec2 {
                        x: marker_x,
                        y: marker_y,
                    } = hex_idx_to_pos(x, y);
                    if let Some(Building {
                        building_type,
                        production_scale: _,
                    }) = tile.building
                    {
                        draw_texture_ex(
                            textures.get(building_type.get_texture_id()).unwrap(),
                            marker_x - 1.2 * HEX_RADIUS / 2.0,
                            marker_y - 1.2 * HEX_RADIUS / 2.0,
                            get_player_color(owner_id),
                            DrawTextureParams {
                                dest_size: Some(Vec2::splat(1.2 * HEX_RADIUS)),
                                ..Default::default()
                            },
                        )
                    } else {
                        draw_circle(marker_x, marker_y, HEX_RADIUS / 4.0, WHITE);
                        draw_circle(
                            marker_x,
                            marker_y,
                            HEX_RADIUS / 5.0,
                            get_player_color(owner_id),
                        );
                    }
                }
                if click && hovering {
                    selected_opt = SelectedAsset::TileIds(x, y);
                }
            }
        }
        for (
            road_id,
            Road {
                tile_a_idx,
                tile_b_idx,
                owner_id,
            },
        ) in game_data.roads.iter().enumerate()
        {
            let is_selected = selected_opt == SelectedAsset::RoadIds(road_id);
            let hovering_road = draw_road(
                *tile_a_idx,
                *tile_b_idx,
                is_selected,
                owner_id.map(|id| get_player_color(id)),
            );
            if click && hovering_road {
                selected_opt = SelectedAsset::RoadIds(road_id);
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
            selected_opt = SelectedAsset::None;
        }

        let mut resource_stock = game_data.get_resource_stock(current_player_id);
        let player_money = resource_stock.get_mut(&Resource::Money).unwrap();
        let PANEL_X = 200.0;
        match selected_opt {
            SelectedAsset::None => (),
            SelectedAsset::RoadIds(road_id) => {
                let road = game_data.roads.get_mut(road_id).unwrap();
                draw_rectangle(PANEL_X, 0.0, 400.0, screen_height(), GRAY);
                draw_text("Road", PANEL_X + MARGIN, MARGIN + 32.0, 32.0, WHITE);
                match road.owner_id {
                    Some(id) if id == current_player_id => {}
                    Some(id) => {}
                    None => {
                        let acquisition_cost = road.get_acquisition_cost();
                        let buy_hovered = draw_button(PANEL_X + MARGIN, 100.0, 150.0, 40.0, RED);
                        draw_text(
                            format!("Buy, ${}", acquisition_cost).as_str(),
                            PANEL_X + 2.0 * MARGIN,
                            120.0,
                            28.0,
                            WHITE,
                        );
                        if buy_hovered
                            && click
                            && player_money.quantity - player_money.net_out().abs()
                                >= acquisition_cost
                        {
                            road.owner_id = Some(current_player_id);
                            *game_data
                                .businesses
                                .get_mut(current_player_id)
                                .unwrap()
                                .resources
                                .get_mut(&backend::Resource::Money)
                                .unwrap() -= acquisition_cost;
                        }
                    }
                }
            }
            SelectedAsset::TileIds(tile_x, tile_y) => {
                let tile = game_data
                    .tiles
                    .get_mut(tile_x)
                    .unwrap()
                    .get_mut(tile_y)
                    .unwrap();
                draw_rectangle(PANEL_X, 0.0, 400.0, screen_height(), GRAY);
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
                        let acquisition_cost = tile.get_acquisition_cost();
                        let buy_hovered = draw_button(PANEL_X + MARGIN, 100.0, 150.0, 40.0, RED);
                        draw_text(
                            format!("Buy, ${}", acquisition_cost).as_str(),
                            PANEL_X + 2.0 * MARGIN,
                            120.0,
                            28.0,
                            WHITE,
                        );
                        if buy_hovered
                            && click
                            && player_money.quantity - player_money.net_out().abs()
                                >= acquisition_cost
                        {
                            tile.owner_id = Some(current_player_id);
                            *game_data
                                .businesses
                                .get_mut(current_player_id)
                                .unwrap()
                                .resources
                                .get_mut(&backend::Resource::Money)
                                .unwrap() -= acquisition_cost;
                        }
                    }
                }
                let mut y_ = 200.0;
                let mut preview_building_type: Option<BuildingType> = None;
                if let Some(building) = tile.building.as_mut() {
                    draw_text(
                        format!("{:?}", building.building_type).as_str(),
                        PANEL_X + MARGIN,
                        y_,
                        32.0,
                        WHITE,
                    );
                    draw_recipes_panel(
                        PANEL_X,
                        y_,
                        building,
                        &resource_stock,
                        &textures,
                        tile.owner_id == Some(current_player_id),
                    );
                } else {
                    draw_text("Available buildings:", PANEL_X + MARGIN, y_, 32.0, WHITE);
                    y_ += 40.0;
                    for building_type in tile.terrain_type.supported_building_types() {
                        draw_text(
                            format!("{:?}", building_type).as_str(),
                            PANEL_X + MARGIN,
                            y_,
                            24.0,
                            WHITE,
                        );
                        let construction_cost = building_type.get_construction_cost();
                        if tile.owner_id == Some(current_player_id) && tile.building.is_none() {
                            let hover_build =
                                draw_button(PANEL_X + 300.0, y_ - 20.0, 100.0, 28.0, RED);
                            draw_text(
                                format!("Buy, ${}", construction_cost).as_str(),
                                PANEL_X + 300.0 + MARGIN,
                                y_,
                                22.0,
                                WHITE,
                            );
                            if hover_build
                                && click
                                && player_money.quantity - player_money.net_out().abs()
                                    >= construction_cost
                            {
                                tile.building = Some(Building::new(building_type));
                                *game_data
                                    .businesses
                                    .get_mut(current_player_id)
                                    .unwrap()
                                    .resources
                                    .get_mut(&backend::Resource::Money)
                                    .unwrap() -= construction_cost;
                            }
                        }
                        let hover_preview =
                            draw_button(PANEL_X + 200.0, y_ - 20.0, 90.0, 28.0, RED);
                        if hover_preview {
                            preview_building_type = Some(building_type)
                        }
                        draw_text("Preview", PANEL_X + 200.0 + MARGIN, y_, 22.0, WHITE);
                        y_ += 32.0;
                    }
                    if let Some(building_type) = preview_building_type {
                        draw_recipes_panel(
                            PANEL_X,
                            y_ + MARGIN,
                            &mut Building::new(building_type),
                            &resource_stock,
                            &textures,
                            false,
                        );
                    }
                }
            }
        }
        next_frame().await
    }
}
