use std::collections::HashMap;

use bevy::prelude::Resource;

#[derive(PartialEq, Eq, Hash)]
enum Material {
    Gold,
    Energy,
    Worker,
    Engineer,
    Chip,
    Wire,
    Computer,
    Log,
    Plank,
    Furniture,
}

pub struct Recipe {
    pub materials: HashMap<Material, i32>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ValidRecipe {
    ComputerAssembly,
    PlankProduction,
    FurnitureProduction,
}

impl ValidRecipe {
    pub fn get_recipe(&self) -> Recipe {
        match self {
            Self::ComputerAssembly => Recipe {
                materials: (vec![
                    (Material::Chip, 1),
                    (Material::Wire, 1),
                    (Material::Computer, -1),
                ])
                .into_iter()
                .collect(),
            },
            Self::PlankProduction => Recipe {
                materials: (vec![
                    (Material::Energy, 1),
                    (Material::Log, 1),
                    (Material::Plank, -1),
                ])
                .into_iter()
                .collect(),
            },
            Self::FurnitureProduction => Recipe {
                materials: (vec![(Material::Plank, 1), (Material::Furniture, -1)])
                    .into_iter()
                    .collect(),
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    WoodWorkingFactory,
    ComputerFactory,
}

impl BuildingType {
    fn get_valid_recipes(&self) -> Vec<ValidRecipe> {
        match self {
            Self::ComputerFactory => vec![ValidRecipe::ComputerAssembly],
            Self::WoodWorkingFactory => vec![
                ValidRecipe::PlankProduction,
                ValidRecipe::FurnitureProduction,
            ],
        }
    }
}

pub struct OwnedBuilding {
    pub building_type: BuildingType,
    pub production_scale: HashMap<ValidRecipe, u32>,
    pub owner_id: Option<usize>,
}

impl OwnedBuilding {
    pub fn new(building_type: BuildingType) -> OwnedBuilding {
        Self {
            building_type: building_type,
            production_scale: building_type
                .get_valid_recipes()
                .into_iter()
                .map(|valid_recipe| (valid_recipe, 0))
                .collect(),
            owner_id: None,
        }
    }
}

pub struct City {
    pub x: f32,
    pub y: f32,
    pub owned_buildings: Vec<OwnedBuilding>,
}

pub struct OwnedConnection {
    pub city_ids: Vec<usize>,
    pub owner_id: Option<usize>,
}

#[derive(Default, Resource)]
pub struct Graph {
    pub cities: Vec<City>,
    pub connections: Vec<OwnedConnection>,
}

#[derive(Default)]
struct Business {
    capital: i32,
    active_recipes: Vec<ValidRecipe>,
}

impl Graph {
    fn get_net_products(&self, business_id: usize, resource: Material) -> i32 {
        self.cities
            .iter()
            .flat_map(|city| city.owned_buildings.iter())
            .filter(|owned_building| owned_building.owner_id == Some(business_id))
            .flat_map(|owned_building| {
                owned_building
                    .production_scale
                    .iter()
                    .map(|(valid_recipe, scale)| {
                        *valid_recipe
                            .get_recipe()
                            .materials
                            .get(&resource)
                            .unwrap_or(&0i32)
                            * (*scale as i32)
                    })
            })
            .sum()
    }

    fn can_buy_building(&self, business_id: usize, city_id: usize, building_id: usize) -> bool {
        let building = self
            .cities
            .get(city_id)
            .unwrap()
            .owned_buildings
            .get(building_id)
            .unwrap();

        building.owner_id == None
            && self.connections.iter().any(|owned_connection| {
                owned_connection.owner_id == Some(business_id)
                    && owned_connection.city_ids.contains(&city_id)
            })
    }

    fn can_buy_connection(&self, business_id: usize, connection_id: usize) -> bool {
        let connection = self.connections.get(connection_id).unwrap();

        connection.owner_id == None
            && self.connections.iter().any(|owned_connection| {
                owned_connection.owner_id == Some(business_id)
                    && owned_connection
                        .city_ids
                        .iter()
                        .any(|city_id| connection.city_ids.contains(city_id))
            })
    }
}
