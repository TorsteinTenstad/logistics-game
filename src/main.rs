use bevy::prelude::*;

mod backend;
use backend::{BuildingType, City, Graph, OwnedBuilding, OwnedConnection};
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let graph = Graph {
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
        ],
    };

    spawn::spawn_building_ui(&mut commands, &asset_server);
    spawn::spawn_graph(
        &graph,
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
    )
}

fn update() {}
