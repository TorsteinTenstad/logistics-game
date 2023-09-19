use backend::{City, Graph, OwnedBuilding};
use bevy::prelude::*;
mod backend;
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let graph = Graph {
        cities: vec![City {
            x: 0.0,
            y: 0.0,
            owned_buildings: vec![OwnedBuilding {
                building_type: backend::BuildingType::ComputerFactory,
                production_scale: (vec![
                    (backend::ValidRecipe::PlankProduction, 1u32),
                    (backend::ValidRecipe::FurnitureProduction, 0u32),
                ])
                .into_iter()
                .collect(),
                owner_id: Some(0),
            }],
        }],
        connections: vec![],
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
