use macroquad::prelude::*;
use std::{collections::HashMap, default};

mod backend;
mod ui;
use backend::{BuildingType, Business, City, Graph, OwnedBuilding, OwnedConnection};
use ui::*;

enum SelectedAsset {
    Building((Vec2, usize, usize)),
    Connection((Vec2, usize)),
    None,
}

#[macroquad::main("logistics-game")]
async fn main() {
    let current_user_business_id = 0;
    let mut graph = Graph {
        cities: vec![
            City {
                x: 260.0,
                y: 60.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
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
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                    OwnedBuilding::new(BuildingType::ComputerFactory),
                ],
            },
            City {
                x: 600.0,
                y: 280.0,
                owned_buildings: vec![
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
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

    let mut selected_asset: SelectedAsset = SelectedAsset::None;
    let mut ui_click_registered;

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

        ui_click_registered = false;
        let mouse_button_pressed = is_mouse_button_pressed(MouseButton::Left);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);
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
            if mouse_button_pressed
                && local_mouse_pos.x.abs() < connection_width / 2.0
                && local_mouse_pos.y > 0.0
                && local_mouse_pos.y < v.length()
            {
                println!("Clicked connection {}", connection_id);
                ui_click_registered = true;
                selected_asset = SelectedAsset::Connection((
                    Vec2::new((start_x + end_x) / 2.0, (start_y + end_y) / 2.0),
                    connection_id,
                ))
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
                if mouse_button_pressed && (mouse_pos - building_pos).length() < building_radius {
                    println!("Clicked building {}", building_id);
                    ui_click_registered = true;
                    selected_asset = SelectedAsset::Building((building_pos, city_id, building_id))
                }
            }
        }

        let x_ = MARGIN;
        let mut y_ = MARGIN;
        let delta = graph.get_resource_delta(current_user_business_id);
        for (material, quantity) in graph
            .businesses
            .get(current_user_business_id)
            .unwrap()
            .resources
            .iter()
        {
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
                format!("{}/{}", quantity, delta.get(&material).unwrap_or(&0)).as_str(),
                x_ + ICON_SIZE + MARGIN,
                y_ + ICON_SIZE / 2.0,
                24.0,
                WHITE,
            );
            y_ += ICON_SIZE + MARGIN;
        }

        let update = draw_buy_ui(screen_width() - 200.0, 200.0);
        if update {
            graph.update_business_resources(current_user_business_id);
        }

        match selected_asset {
            SelectedAsset::None => {}
            SelectedAsset::Building((building_pos, city_id, building_id)) => {
                let mut building = graph
                    .cities
                    .get_mut(city_id)
                    .unwrap()
                    .owned_buildings
                    .get_mut(building_id)
                    .unwrap();

                let x: f32 = building_pos.x;
                let y = building_pos.y;

                match building.owner_id {
                    Some(id) if id == current_user_business_id => {
                        let mut texture_ids: Vec<String> = vec!["right_arrow".to_string()];
                        for (material, quantity) in building
                            .production_scale
                            .get(0)
                            .unwrap()
                            .valid_recipe
                            .get_recipe()
                            .materials
                            .iter()
                        {
                            let index = if *quantity < 0 { texture_ids.len() } else { 0 };
                            texture_ids.insert(index, material.get_texture_id());
                        }

                        let w = texture_ids.len() as f32 * (TEXTURE_SIZE + MARGIN)
                            + 2.0 * MARGIN
                            + 50.0;
                        let h = TEXTURE_SIZE + 2.0 * MARGIN;

                        draw_rectangle(x, y, w, h, UI_BACKGROUND_COLOR);

                        let mut x_ = x + MARGIN;
                        let mut y_ = y + MARGIN;

                        let click_up =
                            draw_rectangle_with_click_detection(x_, y_, 50.0, 25.0, BLACK);
                        let click_down = draw_rectangle_with_click_detection(
                            x_,
                            y + h - MARGIN - 25.0,
                            50.0,
                            25.0,
                            BLACK,
                        );
                        let scaled_valid_recipe = building.production_scale.get_mut(0).unwrap();
                        match (click_up, click_down) {
                            (true, false) => scaled_valid_recipe.scale += 1,
                            (false, true) => scaled_valid_recipe.scale -= 1,
                            _ => (),
                        };
                        ui_click_registered |= click_up || click_down;
                        draw_text(
                            format!("{}", scaled_valid_recipe.scale).as_str(),
                            x_ + 25.0,
                            y_ + h / 2.0,
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
                                let clicked = draw_buy_ui(x, y);
                                if clicked && !ui_click_registered {
                                    ui_click_registered = true;
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
                                        .get_mut(&backend::Material::Gold)
                                        .unwrap() -= building.acquisition_cost;
                                }
                            }
                            false => {
                                draw_message_box_ui(x, y, "Not connected\nto your network");
                            }
                        }
                    }
                }
            }
            SelectedAsset::Connection((connection_pos, connection_id)) => {
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
                    Some(id) if id == current_user_business_id => {}
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
                                let clicked = draw_buy_ui(x, y);
                                if clicked && !ui_click_registered {
                                    let owned_connection =
                                        graph.connections.get_mut(connection_id).unwrap();
                                    ui_click_registered = true;
                                    owned_connection.owner_id = Some(current_user_business_id);
                                    *graph
                                        .businesses
                                        .get_mut(current_user_business_id)
                                        .unwrap()
                                        .resources
                                        .get_mut(&backend::Material::Gold)
                                        .unwrap() -= owned_connection.acquisition_cost;
                                }
                            }
                            false => {
                                draw_message_box_ui(x, y, "Not connected\nto your network");
                            }
                        }
                    }
                }
            }
        }
        if mouse_button_pressed && !ui_click_registered {
            selected_asset = SelectedAsset::None;
        }
        next_frame().await
    }
}
