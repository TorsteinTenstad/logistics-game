use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}
fn startup(mut child_builder: Commands, asset_server: Res<AssetServer>) {
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
                            for texture_name in
                                vec!["chip", "add", "wire", "right_arrow", "computer"]
                            {
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
