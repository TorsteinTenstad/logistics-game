use macroquad::prelude::*;
use std::{collections::HashMap, default};

mod backend;
mod ui;
use backend::{
    BuildingType, Business, City, Graph, OwnedBuilding, OwnedConnection, ScaledValidRecipe,
};
use ui::*;

enum Asset {
    Building((usize, usize)),
    Connection(usize),
}

struct AssetUI {
    pub asset: Asset,
    pub position: Vec2,
    pub size: Option<Vec2>,
}

#[macroquad::main("logistics-game")]
async fn main() {
    let current_user_business_id = 0;
    let mut graph = Graph {
        cities: vec![
            City {
                x: 260.0,
                y: 60.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::Market)],
            },
            City {
                x: 500.0,
                y: 100.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
            },
            City {
                x: 280.0,
                y: 300.0,
                owned_buildings: vec![
                    OwnedBuilding::new(BuildingType::SandPlant),
                    OwnedBuilding::new(BuildingType::Mine),
                ],
            },
            City {
                x: 600.0,
                y: 280.0,
                owned_buildings: vec![
                    OwnedBuilding::new(BuildingType::MetalRefinery),
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                    OwnedBuilding::new(BuildingType::ComputerFactory),
                ],
            },
            City {
                x: 400.0,
                y: 380.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
            },
        ],
        connections: vec![
            OwnedConnection {
                city_ids: vec![0, 1],
                owner_id: None,
                acquisition_cost: 50,
            },
            OwnedConnection {
                city_ids: vec![0, 2],
                owner_id: None,
                acquisition_cost: 50,
            },
            OwnedConnection {
                city_ids: vec![0, 3],
                owner_id: None,
                acquisition_cost: 50,
            },
            OwnedConnection {
                city_ids: vec![2, 4],
                owner_id: None,
                acquisition_cost: 50,
            },
        ],
        businesses: vec![Business::new()],
    };

    graph
        .cities
        .get_mut(0)
        .unwrap()
        .owned_buildings
        .get_mut(0)
        .unwrap()
        .owner_id = Some(current_user_business_id);

    let mut open_asset_ui_opt: Option<AssetUI> = None;

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
    ] {
        textures.insert(
            texture_id.to_string(),
            load_texture(format!("assets/textures/{}.png", texture_id).as_str())
                .await
                .unwrap(),
        );
    }

    request_new_screen_size(1920.0, 1080.0);

    loop {
        clear_background(BLACK);

        let mouse_button_pressed = is_mouse_button_pressed(MouseButton::Left);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);
        let cursor_inside_asset_ui = open_asset_ui_opt.as_ref().is_some_and(|ui| {
            let relative_mouse_pos = mouse_pos - ui.position;
            relative_mouse_pos.cmpgt(Vec2::ZERO).all()
                && relative_mouse_pos.cmplt(ui.size.unwrap()).all()
        });
        if cursor_inside_asset_ui {
            match open_asset_ui_opt.as_ref().unwrap().asset {
                Asset::Building(_) => println!("In building ui"),
                Asset::Connection(_) => println!("In connection ui"),
            }
        }

        let mut city_positions = HashMap::<usize, (f32, f32)>::new();
        for (city_id, city) in graph.cities.iter().enumerate() {
            city_positions.insert(city_id, (city.x, city.y));
        }
        for (connection_id, owned_connection) in graph.connections.iter().enumerate() {
            let connection_width = 10.0;
            assert!(owned_connection.city_ids.len() == 2);
            let (start_x, start_y) = city_positions
                .get(owned_connection.city_ids.get(0).unwrap())
                .unwrap()
                .clone();
            let (end_x, end_y) = city_positions
                .get(owned_connection.city_ids.get(1).unwrap())
                .unwrap()
                .clone();
            let v = Vec2::new(end_x - start_x, end_y - start_y);
            let angle = v.angle_between(Vec2::new(0.0, 1.0));
            draw_rectangle_ex(
                start_x,
                start_y,
                connection_width,
                v.length(),
                DrawRectangleParams {
                    rotation: -angle,
                    offset: Vec2::new(0.5, 0.0),
                    color: if owned_connection.owner_id == Some(current_user_business_id) {
                        BLUE
                    } else {
                        GRAY
                    },
                    ..Default::default()
                },
            );

            let local_mouse_pos =
                Vec2::from_angle(angle).rotate(mouse_pos - Vec2::new(start_x, start_y));
            if !cursor_inside_asset_ui
                && local_mouse_pos.x.abs() < connection_width / 2.0
                && local_mouse_pos.y > 0.0
                && local_mouse_pos.y < v.length()
            {
                println!("Clicked connection {}", connection_id);
                open_asset_ui_opt = Some(AssetUI {
                    asset: Asset::Connection(connection_id),
                    position: Vec2::new((start_x + end_x) / 2.0, (start_y + end_y) / 2.0),
                    size: None,
                });
            }
        }

        for (city_id, city) in graph.cities.iter().enumerate() {
            let city_radius = 50.0;
            let building_radius = 10.0;

            draw_hexagon(
                city.x,
                city.y,
                city_radius,
                0.0,
                true,
                WHITE,
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            );

            for (building_id, owned_building) in city.owned_buildings.iter().enumerate() {
                let rad = 2.0 * std::f32::consts::PI * (building_id as f32 / 6.0);
                let building_pos = Vec2::new(
                    0.5 * city_radius * f32::sin(rad) + city.x,
                    0.5 * city_radius * f32::cos(rad) + city.y,
                );
                draw_circle(
                    building_pos.x,
                    building_pos.y,
                    building_radius,
                    if owned_building.owner_id == Some(current_user_business_id) {
                        BLUE
                    } else {
                        GRAY
                    },
                );
                if !cursor_inside_asset_ui && (mouse_pos - building_pos).length() < building_radius
                {
                    println!("Clicked building {}", building_id);
                    open_asset_ui_opt = Some(AssetUI {
                        asset: Asset::Building((city_id, building_id)),
                        position: building_pos,
                        size: None,
                    });
                }
            }
        }

        let x_ = MARGIN;
        let mut y_ = MARGIN;
        for (material, quantity_info) in graph.get_resource_stock(current_user_business_id) {
            draw_texture_ex(
                textures.get(&material.get_texture_id()).unwrap(),
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
                    "{}/{}",
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
        if ButtonState::Pressed == draw_next_turn_button(screen_width() - 200.0, 200.0).0 {
            graph.update_business_resources(current_user_business_id);
        }

        if let Some(open_asset_ui) = open_asset_ui_opt.as_mut() {
            match open_asset_ui.asset {
                Asset::Building((city_id, building_id)) => {
                    let resource_stock = graph.get_resource_stock(current_user_business_id);
                    let building = graph
                        .cities
                        .get_mut(city_id)
                        .unwrap()
                        .owned_buildings
                        .get_mut(building_id)
                        .unwrap();

                    let x: f32 = open_asset_ui.position.x;
                    let y = open_asset_ui.position.y;

                    match building.owner_id {
                        Some(id) if id == current_user_business_id => {
                            let mut x_ = x + MARGIN;
                            let mut y_ = y + MARGIN;

                            let w = 5 as f32 * (TEXTURE_SIZE + MARGIN) + 2.0 * MARGIN + 50.0;
                            let h = (TEXTURE_SIZE + MARGIN)
                                * building.production_scale.len() as f32
                                + MARGIN;
                            open_asset_ui.size = Some(Vec2::new(w, h));
                            draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);

                            for ScaledValidRecipe {
                                valid_recipe,
                                scale,
                            } in building.production_scale.iter_mut()
                            {
                                let mut texture_ids: Vec<String> = vec!["right_arrow".to_string()];

                                for (material, quantity) in
                                    valid_recipe.get_recipe().materials.iter()
                                {
                                    let index = if *quantity > 0 { texture_ids.len() } else { 0 };
                                    texture_ids.insert(index, material.get_texture_id());
                                }

                                let click_up = ButtonState::Pressed
                                    == draw_button(x_, y_, 50.0, 25.0, BLACK).0;
                                let click_down = ButtonState::Pressed
                                    == draw_button(
                                        x_,
                                        y_ + TEXTURE_SIZE - MARGIN - 25.0,
                                        50.0,
                                        25.0,
                                        BLACK,
                                    )
                                    .0;
                                let requested_increment = match (click_up, click_down, *scale == 0)
                                {
                                    (true, false, _) => 1,
                                    (false, true, false) => -1,
                                    _ => 0,
                                };
                                let can_increment = requested_increment != 0
                                    && valid_recipe.get_recipe().materials.iter().all(
                                        |(material, quantity)| match resource_stock.get(material) {
                                            Some(quantity_info) => {
                                                quantity_info.quantity
                                                    + requested_increment * quantity
                                                    + quantity_info.gross_in
                                                    - quantity_info.gross_out
                                                    >= 0
                                            }
                                            None => requested_increment * *quantity >= 0,
                                        },
                                    );
                                if can_increment {
                                    *scale += requested_increment;
                                }
                                draw_text(
                                    format!("{}", scale).as_str(),
                                    x_ + 25.0,
                                    y_ + TEXTURE_SIZE / 2.0,
                                    32.0,
                                    WHITE,
                                );
                                x_ += 50.0 + MARGIN;

                                for texture_id in texture_ids {
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
                                    x_ += TEXTURE_SIZE + MARGIN;
                                }
                                x_ = x + MARGIN;
                                y_ += TEXTURE_SIZE + MARGIN;
                            }
                        }
                        Some(other_user_business_id) => {
                            todo!()
                        }
                        None => {
                            let can_buy = graph.connections.iter().any(|owned_connection| {
                                owned_connection.owner_id == Some(current_user_business_id)
                                    && owned_connection.city_ids.contains(&city_id)
                            });

                            match can_buy {
                                true => {
                                    let (buy_ui_state, size) = draw_buy_ui(x, y);
                                    open_asset_ui.size = Some(size);
                                    if buy_ui_state == ButtonState::Pressed {
                                        let building = graph
                                            .cities
                                            .get_mut(city_id)
                                            .unwrap()
                                            .owned_buildings
                                            .get_mut(building_id)
                                            .unwrap();
                                        building.owner_id = Some(current_user_business_id);
                                        *graph
                                            .businesses
                                            .get_mut(current_user_business_id)
                                            .unwrap()
                                            .resources
                                            .get_mut(&backend::Material::Money)
                                            .unwrap() -= building.acquisition_cost;
                                    }
                                }
                                false => {
                                    let size =
                                        draw_message_box_ui(x, y, "Not connected\nto your network");
                                    open_asset_ui.size = Some(size);
                                }
                            }
                        }
                    }
                }
                Asset::Connection(connection_id) => {
                    let owned_connection = graph.connections.get(connection_id).unwrap();

                    let texture_size = 100.0;
                    let (start_x, start_y) = city_positions
                        .get(owned_connection.city_ids.get(0).unwrap())
                        .unwrap()
                        .clone();
                    let (end_x, end_y) = city_positions
                        .get(owned_connection.city_ids.get(1).unwrap())
                        .unwrap()
                        .clone();
                    let x = (start_x + end_x) / 2.0;
                    let y = (start_y + end_y) / 2.0;
                    let margin = 10.0;
                    let ui_background_color = Color {
                        r: 0.4,
                        g: 0.4,
                        b: 0.4,
                        a: 1.0,
                    };
                    let mut x_ = x + margin;
                    let mut y_ = y + margin;

                    match owned_connection.owner_id {
                        Some(id) if id == current_user_business_id => {
                            let size = draw_message_box_ui(x, y, "Maintenance cost: 0");
                            open_asset_ui.size = Some(size);
                        }
                        Some(other_user_business_id) => {
                            todo!()
                        }
                        None => {
                            let can_buy =
                                graph.connections.iter().any(|owned_connection| {
                                    owned_connection.owner_id == Some(current_user_business_id)
                                        && owned_connection.city_ids.iter().any(|city_id| {
                                            graph
                                                .connections
                                                .get(connection_id)
                                                .unwrap()
                                                .city_ids
                                                .contains(city_id)
                                        })
                                }) || graph.cities.iter().enumerate().any(|(city_it_id, city)| {
                                    city.owned_buildings.iter().any(|owned_building| {
                                        owned_building.owner_id == Some(current_user_business_id)
                                            && owned_connection.city_ids.iter().any(|city_id| {
                                                graph
                                                    .connections
                                                    .get(connection_id)
                                                    .unwrap()
                                                    .city_ids
                                                    .contains(&city_it_id)
                                            })
                                    })
                                });

                            match can_buy {
                                true => {
                                    let (clicked, size) = draw_buy_ui(x, y);
                                    open_asset_ui.size = Some(size);
                                    if clicked == ButtonState::Pressed {
                                        let owned_connection =
                                            graph.connections.get_mut(connection_id).unwrap();
                                        owned_connection.owner_id = Some(current_user_business_id);
                                        *graph
                                            .businesses
                                            .get_mut(current_user_business_id)
                                            .unwrap()
                                            .resources
                                            .get_mut(&backend::Material::Money)
                                            .unwrap() -= owned_connection.acquisition_cost;
                                    }
                                }
                                false => {
                                    let size =
                                        draw_message_box_ui(x, y, "Not connected\nto your network");
                                    open_asset_ui.size = Some(size);
                                }
                            }
                        }
                    }
                }
            }
        }
        if mouse_button_pressed && !cursor_inside_asset_ui {
            open_asset_ui_opt = None;
        }
        next_frame().await
    }
}
