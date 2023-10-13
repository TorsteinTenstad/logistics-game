use macroquad::prelude::*;
use std::collections::HashMap;

mod backend;
use backend::{BuildingType, City, Graph, OwnedBuilding, OwnedConnection};

#[macroquad::main("logistics-game")]
async fn main() {
    let graph = Graph {
        cities: vec![
            City {
                x: 60.0,
                y: 60.0,
                owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
            },
            City {
                x: 300.0,
                y: 60.0,
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
                y: 180.0,
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
            },
            OwnedConnection {
                city_ids: vec![0, 2],
                owner_id: None,
            },
            OwnedConnection {
                city_ids: vec![0, 3],
                owner_id: None,
            },
            OwnedConnection {
                city_ids: vec![2, 4],
                owner_id: None,
            },
        ],
    };

    loop {
        clear_background(BLACK);

        let mouse_button_down = is_mouse_button_pressed(MouseButton::Left);
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);
        let mut city_positions = HashMap::<usize, Vec2>::new();
        for (city_id, city) in graph.cities.iter().enumerate() {
            let city_radius = 50.0;
            let building_radius = 10.0;
            let position = Vec2::new(city.x, city.y);
            city_positions.insert(city_id, position);

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
                draw_circle(building_pos.x, building_pos.y, building_radius, BLUE);
                if mouse_button_down && (mouse_pos - building_pos).length() < building_radius {
                    println!("Clicked building {}", building_id);
                }
            }
        }
        for (connection_id, owned_connection) in graph.connections.iter().enumerate() {
            let connection_width = 10.0;
            assert!(owned_connection.city_ids.len() == 2);
            let start = city_positions
                .get(owned_connection.city_ids.get(0).unwrap())
                .unwrap()
                .clone();
            let end = city_positions
                .get(owned_connection.city_ids.get(1).unwrap())
                .unwrap()
                .clone();

            draw_line(start.x, start.y, end.x, end.y, connection_width, WHITE)
        }
        next_frame().await
    }
}
