use std::collections::HashMap;

use bevy::ecs::query::Has;

#[derive(PartialEq, Eq, Hash)]
enum Resource {
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

struct Recipe {
    materials: HashMap<Resource, i32>,
}

enum ValidRecipe {
    ComputerAssembly,
    PlankProduction,
    FurnitureProduction,
}

impl ValidRecipe {
    fn get_recipe(&self) -> Recipe {
        match self {
            Self::ComputerAssembly => Recipe {
                materials: (vec![
                    (Resource::Chip, 1),
                    (Resource::Wire, 1),
                    (Resource::Computer, -1),
                ])
                .into_iter()
                .collect(),
            },
            Self::PlankProduction => Recipe {
                materials: (vec![
                    (Resource::Energy, 1),
                    (Resource::Log, 1),
                    (Resource::Plank, -1),
                ])
                .into_iter()
                .collect(),
            },
            Self::FurnitureProduction => Recipe {
                materials: (vec![(Resource::Plank, 1), (Resource::Furniture, -1)])
                    .into_iter()
                    .collect(),
            },
        }
    }
}

enum BuildingType {
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

struct OwnedBuilding {
    building_type: BuildingType,
    production_scale: HashMap<Recipe, u32>,
    owner_id: Option<usize>,
}

struct City {
    owned_buildings: Vec<OwnedBuilding>,
}

struct OwnedConnection {
    owner_id: Option<usize>,
    city_ids: Vec<usize>,
}

#[derive(Default)]
struct Graph {
    cities: Vec<City>,
    connections: Vec<OwnedConnection>,
}

#[derive(Default)]
struct Business {
    capital: i32,
    active_recipes: Vec<ValidRecipe>,
}

impl Graph {
    fn get_net_products(&self, business_id: usize, resource: Resource) -> i32 {
        self.cities
            .iter()
            .flat_map(|city| city.owned_buildings.iter())
            .filter(|owned_building| owned_building.owner_id == Some(business_id))
            .flat_map(|owned_building| {
                owned_building
                    .production_scale
                    .iter()
                    .map(|(recipe, scale)| {
                        *recipe.materials.get(&resource).unwrap_or(&0i32) * (*scale as i32)
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

fn main() {
    let business = Business::default();
}
