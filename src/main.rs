use macroquad::prelude::*;
use std::collections::HashMap;

mod backend;
mod ui;
use backend::{BuildingType, Business, City, Graph, OwnedBuilding, OwnedConnection};
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

fn get_player_color(player_id: usize) -> Color {
    match player_id {
        0 => BLUE,
        1 => GREEN,
        2 => YELLOW,
        _ => todo!(),
    }
}

#[macroquad::main("logistics-game")]
async fn main() {
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
        businesses: vec![Business::new(), Business::new()],
    };

    let mut current_player_id = 0;

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

        let owns_nothing = graph
            .connections
            .iter()
            .all(|owned_connection| owned_connection.owner_id != Some(current_player_id))
            && graph
                .cities
                .iter()
                .flat_map(|city| city.owned_buildings.iter())
                .all(|owned_building| owned_building.owner_id != Some(current_player_id));

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
                    color: owned_connection.owner_id.map_or(GRAY, get_player_color),
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
                    owned_building.owner_id.map_or(GRAY, get_player_color),
                );
                if !cursor_inside_asset_ui && (mouse_pos - building_pos).length() < building_radius
                {
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
        draw_button(x_, y_, 100.0, MARGIN, get_player_color(current_player_id));
        y_ += MARGIN + MARGIN;
        for (material, quantity_info) in graph.get_resource_stock(current_player_id) {
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
        if ButtonState::Pressed == draw_next_turn_button(screen_width() - 200.0, MARGIN).0 {
            current_player_id += 1;
            if graph.businesses.get(current_player_id).is_none() {
                current_player_id = 0;
            }
            graph.update_business_resources(current_player_id);
        }

        if let Some(open_asset_ui) = open_asset_ui_opt.as_mut() {
            open_asset_ui.size = Some(match open_asset_ui.asset {
                Asset::Building((city_id, building_id)) => {
                    let resource_stock = graph.get_resource_stock(current_player_id);
                    let building = graph
                        .cities
                        .get_mut(city_id)
                        .unwrap()
                        .owned_buildings
                        .get_mut(building_id)
                        .unwrap();

                    let x = open_asset_ui.position.x;
                    let y = open_asset_ui.position.y;

                    match building.owner_id {
                        Some(id) if id == current_player_id => {
                            draw_recipes_panel(x, y, building, &resource_stock, &textures, true)
                        }
                        Some(other_id) => {
                            draw_message_box_ui(x, y, format!("Owned by {}", other_id).as_str())
                        }
                        None => {
                            let can_buy = owns_nothing
                                || graph.connections.iter().any(|owned_connection| {
                                    owned_connection.owner_id == Some(current_player_id)
                                        && owned_connection.city_ids.contains(&city_id)
                                });

                            match can_buy {
                                true => {
                                    let (buy_ui_state, size) =
                                        draw_buy_ui(x, y, building.acquisition_cost);
                                    if buy_ui_state == ButtonState::Pressed {
                                        building.owner_id = Some(current_player_id);
                                        *graph
                                            .businesses
                                            .get_mut(current_player_id)
                                            .unwrap()
                                            .resources
                                            .get_mut(&backend::Material::Money)
                                            .unwrap() -= building.acquisition_cost;
                                    }
                                    draw_recipes_panel(
                                        x,
                                        y + size.y,
                                        building,
                                        &resource_stock,
                                        &textures,
                                        false,
                                    ) + Vec2::new(0.0, size.y)
                                }
                                false => {
                                    let size =
                                        draw_message_box_ui(x, y, "Not connected\nto your network");

                                    draw_recipes_panel(
                                        x,
                                        y + size.y,
                                        building,
                                        &resource_stock,
                                        &textures,
                                        false,
                                    ) + Vec2::new(0.0, size.y)
                                }
                            }
                        }
                    }
                }
                Asset::Connection(connection_id) => {
                    let owned_connection = graph.connections.get(connection_id).unwrap();

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

                    match owned_connection.owner_id {
                        Some(id) if id == current_player_id => {
                            draw_message_box_ui(x, y, "Maintenance cost: 0")
                        }
                        Some(other_id) => {
                            draw_message_box_ui(x, y, format!("Owned by {}", other_id).as_str())
                        }
                        None => {
                            let can_buy =
                                graph.connections.iter().any(|owned_connection| {
                                    owned_connection.owner_id == Some(current_player_id)
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
                                        owned_building.owner_id == Some(current_player_id)
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
                                    let (clicked, size) =
                                        draw_buy_ui(x, y, owned_connection.acquisition_cost);
                                    open_asset_ui.size = Some(size);
                                    if clicked == ButtonState::Pressed {
                                        let owned_connection =
                                            graph.connections.get_mut(connection_id).unwrap();
                                        owned_connection.owner_id = Some(current_player_id);
                                        *graph
                                            .businesses
                                            .get_mut(current_player_id)
                                            .unwrap()
                                            .resources
                                            .get_mut(&backend::Material::Money)
                                            .unwrap() -= owned_connection.acquisition_cost;
                                    }
                                    size
                                }
                                false => {
                                    draw_message_box_ui(x, y, "Not connected\nto your network")
                                }
                            }
                        }
                    }
                }
            })
        }
        if mouse_button_pressed && !cursor_inside_asset_ui {
            open_asset_ui_opt = None;
        }
        next_frame().await
    }
}
