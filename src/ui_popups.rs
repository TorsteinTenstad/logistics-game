use super::backend::Graph;
use super::id_components::{BuildingComponent, ConnectionComponent};
use super::mouse_detector::{MouseDetector, MouseDetectorState};
use bevy::prelude::*;

#[derive(Component)]
pub struct UiPopup {}

pub fn update_ui_popups(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    asset_server: Res<AssetServer>,
    graph: Res<Graph>,
    query_buildings: Query<
        (&MouseDetector, &BuildingComponent, &GlobalTransform),
        Changed<MouseDetector>,
    >,
    query_connections: Query<
        (&MouseDetector, &ConnectionComponent, &GlobalTransform),
        Changed<MouseDetector>,
    >,
    query_ui_popups: Query<Entity, With<UiPopup>>,
) {
    for (mouse_detector, building_component, global_transform) in &query_buildings {
        match mouse_detector.detector_state {
            MouseDetectorState::None => (),
            MouseDetectorState::Hover => (),
            MouseDetectorState::Press => {
                let (camera, camera_global_transform) = camera_q.single();
                let spawn_viewport_pos = camera
                    .world_to_viewport(
                        camera_global_transform,
                        global_transform.compute_transform().translation,
                    )
                    .unwrap();
                despawn_building_ui(&mut commands, &query_ui_popups);
                spawn_building_ui(
                    &mut commands,
                    &asset_server,
                    &graph,
                    building_component,
                    spawn_viewport_pos,
                );
            }
        }
    }
    if mouse_buttons.just_pressed(MouseButton::Left) && !query_buildings.iter().any(|_| true) {
        despawn_building_ui(&mut commands, &query_ui_popups);
    }
}

fn despawn_building_ui(commands: &mut Commands, query: &Query<Entity, With<UiPopup>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_building_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    graph: &Graph,
    building_component: &BuildingComponent,
    position: Vec2,
) {
    graph
        .cities
        .get(building_component.city_id)
        .unwrap()
        .owned_buildings
        .get(building_component.building_id)
        .unwrap()
        .production_scale
        .iter()
        .map(|(valid_recipe, production_scale)| ());

    let child_builder = commands;
    child_builder
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(position.y),
                    left: Val::Px(position.x),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.4, 0.4, 1.).into(),
                ..default()
            },
            UiPopup {},
        ))
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
