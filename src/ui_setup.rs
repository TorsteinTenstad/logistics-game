use super::id_components;
use super::mouse_detector;
use crate::backend::Graph;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::collections::HashMap;
use std::*;

fn atan2(vec2: Vec2) -> f32 {
    f32::atan2(vec2.x, vec2.y)
}

enum ZOrder {
    Connection,
    City,
    Building,
}
impl From<ZOrder> for f32 {
    fn from(value: ZOrder) -> Self {
        value as i32 as f32
    }
}

pub fn setup_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graph: Res<Graph>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut city_positions = HashMap::<usize, Vec2>::new();
    for (city_id, city) in graph.cities.iter().enumerate() {
        let city_radius = 50.0;
        let building_radius = 10.0;
        let position = Vec2::new(city.x, city.y);
        city_positions.insert(city_id, position);
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(city_radius, 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(city.x, city.y, ZOrder::City.into())),
            ..default()
        },));
        for (building_id, owned_building) in city.owned_buildings.iter().enumerate() {
            let rad = 2.0 * f32::consts::PI * (building_id as f32 / 6.0);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(building_radius).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    transform: Transform::from_translation(Vec3::new(
                        0.5 * city_radius * f32::sin(rad) + city.x,
                        0.5 * city_radius * f32::cos(rad) + city.y,
                        ZOrder::Building.into(),
                    )),
                    ..default()
                },
                mouse_detector::MouseDetector::new(building_radius, building_radius),
                id_components::BuildingComponent {
                    city_id: city_id,
                    building_id: building_id,
                },
            ));
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
        let midpoint = (start + end) / 2f32;
        let connection_length = Vec2::distance(start, end);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(connection_width, connection_length)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    midpoint.x,
                    midpoint.y,
                    ZOrder::Connection.into(),
                ))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, -atan2(end - start))),
                ..default()
            },
            mouse_detector::MouseDetector::new(connection_width, connection_length),
            id_components::ConnectionComponent { id: connection_id },
        ));
    }
}
