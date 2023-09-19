use bevy::prelude::*;

mod backend;
mod id_components;
mod mouse_detector;
mod ui_popups;
mod ui_setup;

use backend::{BuildingType, City, Graph, OwnedBuilding, OwnedConnection};

fn main() {
    App::new()
        .insert_resource(Graph {
            cities: vec![
                City {
                    x: 0.0,
                    y: 0.0,
                    owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
                },
                City {
                    x: 300.0,
                    y: 50.0,
                    owned_buildings: vec![OwnedBuilding::new(BuildingType::ComputerFactory)],
                },
                City {
                    x: 200.0,
                    y: -150.0,
                    owned_buildings: vec![
                        OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                        OwnedBuilding::new(BuildingType::ComputerFactory),
                    ],
                },
                City {
                    x: -200.0,
                    y: 180.0,
                    owned_buildings: vec![
                        OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                        OwnedBuilding::new(BuildingType::WoodWorkingFactory),
                        OwnedBuilding::new(BuildingType::ComputerFactory),
                    ],
                },
                City {
                    x: -200.0,
                    y: -180.0,
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
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Startup, ui_setup::setup_ui)
        .add_systems(Update, mouse_detector::update_mouse_detector)
        .add_systems(Update, ui_popups::update_ui_popups)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
