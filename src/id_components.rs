use bevy::prelude::*;

#[derive(Component)]
pub struct BuildingComponent {
    pub id: usize,
}

#[derive(Component)]
pub struct ConnectionComponent {
    pub id: usize,
}
