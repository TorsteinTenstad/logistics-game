use super::backend;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::*;

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
    for city in graph.cities.iter() {
        let city_radius = 50.0;
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(city_radius, 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(city.x, city.y, ZOrder::City.into())),
            ..default()
        });
        for (i, owned_building) in city.owned_buildings.iter().enumerate() {
            let rad = 2.0 * f32::consts::PI * (i as f32 / 6.0);
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(Vec3::new(
                    0.5 * city_radius * f32::sin(rad) + city.x,
                    0.5 * city_radius * f32::cos(rad) + city.y,
                    ZOrder::Building.into(),
                )),
                ..default()
            });
        }
    }
}

pub fn spawn_building_ui(child_builder: &mut Commands, asset_server: &Res<AssetServer>) {
    child_builder.spawn(Camera2dBundle::default());

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
