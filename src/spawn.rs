use super::backend;
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

pub fn spawn_graph(
    graph: &backend::Graph,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let mut city_positions = HashMap::<usize, Vec2>::new();
    for (city_id, city) in graph.cities.iter().enumerate() {
        let city_radius = 50.0;
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
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(
                    0.5 * city_radius * f32::sin(rad) + city.x,
                    0.5 * city_radius * f32::cos(rad) + city.y,
                    ZOrder::Building.into(),
                )),
                ..default()
            },));
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
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(connection_width, Vec2::distance(start, end))),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                midpoint.x,
                midpoint.y,
                ZOrder::Connection.into(),
            ))
            .with_rotation(Quat::from_axis_angle(Vec3::Z, -atan2(end - start))),
            ..default()
        });
    }
}

pub fn spawn_building_ui(child_builder: &mut Commands, asset_server: &Res<AssetServer>) {
    child_builder
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(100.0),
                left: Val::Px(200.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.4, 0.4, 1.).into(),
            ..default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(50.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    });
                    child_builder.spawn((TextBundle::from_section(
                        "5",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    ),));
                    child_builder.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(50.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    });
                });

            child_builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn((TextBundle::from_section(
                        "Computer Assembly",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.,
                            color: Color::WHITE,
                        },
                    ),));
                    child_builder
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|child_builder| {
                            for texture_name in vec!["chip", "wire", "right_arrow", "computer"] {
                                child_builder.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(70.0),
                                            height: Val::Px(70.0),
                                            margin: UiRect::top(Val::VMin(5.)),
                                            ..default()
                                        },
                                        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                        background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    UiImage::new(
                                        asset_server.load(format!("textures/{}.png", texture_name)),
                                    ),
                                ));
                            }
                        });
                });
        });
}
