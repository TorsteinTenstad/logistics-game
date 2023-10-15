use macroquad::prelude::*;
use std::{collections::HashMap, default};

mod backend;
use backend::{BuildingType, Business, City, Graph, OwnedBuilding, OwnedConnection};

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
                x: 60.0,
                y: 60.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
            },
            City {
                x: 300.0,
                y: 100.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
            },
            City {
                x: 80.0,
                y: 300.0,
                owned_buildings: vec![
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                    OwnedBuilding::new(BuildingType::ComputerFactory),
                ],
            },
            City {
                x: 400.0,
                y: 280.0,
                owned_buildings: vec![
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                    OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                    OwnedBuilding::new(BuildingType::ComputerFactory),
                ],
            },
            City {
                x: 200.0,
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
        businesses: vec![Business { capital: 1000 }],
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
    let mut ui_click_registered = false;
    loop {
        clear_background(BLACK);

        draw_text_ex(
            format!(
                "$ {}",
                graph
                    .businesses
                    .get(current_user_business_id)
                    .unwrap()
                    .capital
            )
            .as_str(),
            10.0,
            screen_height() - 10.0,
            TextParams {
                font_size: 32,
                ..Default::default()
            },
        );
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

        match selected_asset {
            SelectedAsset::None => {}
            SelectedAsset::Building((building_pos, city_id, building_id)) => {
                let building = graph
                    .cities
                    .get(city_id)
                    .unwrap()
                    .owned_buildings
                    .get(building_id)
                    .unwrap();

                let texture_size = 100.0;
                let x = building_pos.x;
                let y = building_pos.y;
                let margin = 10.0;
                let ui_background_color = Color {
                    r: 0.4,
                    g: 0.4,
                    b: 0.4,
                    a: 1.0,
                };
                let mut x_ = x + margin;
                let mut y_ = y + margin;

                match building.owner_id {
                    Some(id) if id == current_user_business_id => {
                        let mut texture_ids: Vec<String> = vec!["right_arrow".to_string()];
                        for (material, quantity) in building
                            .production_scale
                            .keys()
                            .next()
                            .unwrap()
                            .get_recipe()
                            .materials
                            .iter()
                        {
                            let index = if *quantity < 0 { texture_ids.len() } else { 0 };
                            let element = match material {
                                backend::Material::Chip => "chip",
                                backend::Material::Gold => "gold",
                                backend::Material::Energy => "energy",
                                backend::Material::Worker => "worker",
                                backend::Material::Engineer => "engineer",
                                backend::Material::Wire => "wire",
                                backend::Material::Computer => "computer",
                                backend::Material::Log => "logs",
                                backend::Material::Plank => "planks",
                                backend::Material::Furniture => "chair",
                            };
                            texture_ids.insert(index, element.to_string());
                        }
                        let w = texture_ids.len() as f32 * (texture_size + margin) + margin;
                        let h = texture_size + 2.0 * margin;

                        draw_rectangle(x, y, w, h, ui_background_color);

                        for texture_id in texture_ids {
                            let texture: Texture2D = load_texture(
                                format!("assets/textures/{}.png", texture_id).as_str(),
                            )
                            .await
                            .unwrap();
                            draw_texture_ex(
                                &texture,
                                x_,
                                y_,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::splat(texture_size)),
                                    ..Default::default()
                                },
                            );
                            x_ += texture_size + margin;
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
                                let w = texture_size + 2.0 * margin;
                                let h = texture_size + 2.0 * margin;
                                draw_rectangle(x, y, w, h, ui_background_color);
                                draw_rectangle(x_, y_, texture_size, texture_size, RED);
                                draw_text("Buy", x_ + margin, y_ + texture_size / 2.0, 32.0, WHITE);
                                let local_mouse_pos = mouse_pos - Vec2::new(x_, y_);
                                if mouse_button_pressed
                                    && local_mouse_pos.cmpgt(Vec2::ZERO).all()
                                    && local_mouse_pos
                                        .cmplt(Vec2::new(texture_size, texture_size))
                                        .all()
                                {
                                    ui_click_registered = true;
                                    let building = graph
                                        .cities
                                        .get_mut(city_id)
                                        .unwrap()
                                        .owned_buildings
                                        .get_mut(building_id)
                                        .unwrap();
                                    building.owner_id = Some(current_user_business_id);
                                    graph
                                        .businesses
                                        .get_mut(current_user_business_id)
                                        .unwrap()
                                        .capital -= building.acquisition_cost;
                                }
                            }
                            false => {
                                let w = 3.5 * texture_size + 2.0 * margin;
                                let h = 0.25 * texture_size + 2.0 * margin;
                                draw_rectangle(x, y, w, h, ui_background_color);
                                draw_text(
                                    "Not connected to\nyour network",
                                    x_ + margin,
                                    y_ + h / 2.0,
                                    24.0,
                                    WHITE,
                                );
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
                                let w = texture_size + 2.0 * margin;
                                let h = texture_size + 2.0 * margin;
                                draw_rectangle(x, y, w, h, ui_background_color);
                                draw_rectangle(x_, y_, texture_size, texture_size, RED);
                                draw_text("Buy", x_ + margin, y_ + texture_size / 2.0, 32.0, WHITE);
                                let local_mouse_pos = mouse_pos - Vec2::new(x_, y_);
                                if mouse_button_pressed
                                    && local_mouse_pos.cmpgt(Vec2::ZERO).all()
                                    && local_mouse_pos
                                        .cmplt(Vec2::new(texture_size, texture_size))
                                        .all()
                                {
                                    let owned_connection =
                                        graph.connections.get_mut(connection_id).unwrap();
                                    ui_click_registered = true;
                                    owned_connection.owner_id = Some(current_user_business_id);
                                    graph
                                        .businesses
                                        .get_mut(current_user_business_id)
                                        .unwrap()
                                        .capital -= owned_connection.acquisition_cost;
                                }
                            }
                            false => {
                                let w = 3.5 * texture_size + 2.0 * margin;
                                let h = 0.25 * texture_size + 2.0 * margin;
                                draw_rectangle(x, y, w, h, ui_background_color);
                                draw_text(
                                    "Not connected to\nyour network",
                                    x_ + margin,
                                    y_ + h / 2.0,
                                    24.0,
                                    WHITE,
                                );
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
