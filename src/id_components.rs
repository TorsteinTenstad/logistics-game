use bevy::prelude::*;

#[derive(Component)]
pub struct BuildingComponent {
    pub city_id: usize,
    pub building_id: usize,
}

#[derive(Component)]
pub struct ConnectionComponent {
    pub id: usize,
}
