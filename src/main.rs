use bevy::prelude::*;
mod backend;
mod id_components;
use backend::{BuildingType, City, Graph, OwnedBuilding, OwnedConnection};
mod mouse_detector;
use id_components::{BuildingComponent, ConnectionComponent};
use mouse_detector::{update_mouse_detector, MouseDetector, MouseDetectorState};
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .add_systems(Update, update_mouse_detector)
        .run();
}
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle { ..default() });

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

    spawn::spawn_graph(
        &graph,
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
    )
}

fn update(
    mut commands: Commands,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    asset_server: Res<AssetServer>,
    query_buildings: Query<
        (&MouseDetector, &BuildingComponent, &GlobalTransform),
        Changed<MouseDetector>,
    >,
    query_connections: Query<
        (&MouseDetector, &ConnectionComponent, &GlobalTransform),
        Changed<MouseDetector>,
    >,
) {
    for (mouse_detector, building_component, global_transform) in &query_buildings {
        match mouse_detector.detector_state {
            MouseDetectorState::Press => {
                let (camera, camera_global_transform) = camera_q.single();
                let spawn_viewport_pos = camera
                    .world_to_viewport(
                        camera_global_transform,
                        global_transform.compute_transform().translation,
                    )
                    .unwrap();
                spawn::spawn_building_ui(&mut commands, &asset_server, spawn_viewport_pos);
            }
            MouseDetectorState::Hover => (),
            MouseDetectorState::None => (),
        }
    }
}
