use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Material {
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

impl Material {
    pub fn get_texture_id(&self) -> String {
        match self {
            Self::Chip => "chip",
            Self::Gold => "gold",
            Self::Energy => "energy",
            Self::Worker => "worker",
            Self::Engineer => "engineer",
            Self::Wire => "wire",
            Self::Computer => "computer",
            Self::Log => "logs",
            Self::Plank => "planks",
            Self::Furniture => "chair",
        }
        .to_string()
    }
}

pub struct Recipe {
    pub materials: Vec<(Material, i32)>,
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
                materials: (vec![(Material::Log, 1), (Material::Plank, -1)])
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

pub struct ScaledValidRecipe {
    pub valid_recipe: ValidRecipe,
    pub scale: u32,
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
                //ValidRecipe::FurnitureProduction,
            ],
        }
    }
}

pub struct OwnedBuilding {
    pub building_type: BuildingType,
    pub production_scale: Vec<ScaledValidRecipe>,
    pub owner_id: Option<usize>,
    pub acquisition_cost: i32,
}

impl OwnedBuilding {
    pub fn new(building_type: BuildingType) -> OwnedBuilding {
        Self {
            building_type: building_type,
            production_scale: building_type
                .get_valid_recipes()
                .into_iter()
                .map(|valid_recipe| ScaledValidRecipe {
                    valid_recipe: valid_recipe,
                    scale: 0,
                })
                .collect(),
            owner_id: None,
            acquisition_cost: 100,
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
    pub acquisition_cost: i32,
}

#[derive(Default)]
pub struct Graph {
    pub cities: Vec<City>,
    pub connections: Vec<OwnedConnection>,
    pub businesses: Vec<Business>,
}

#[derive(Default)]
pub struct Business {
    pub resources: BTreeMap<Material, i32>,
}

impl Business {
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::from([(Material::Gold, 1000)]),
        }
    }
}

impl Graph {
    pub fn get_resource_delta(&self, business_id: usize) -> BTreeMap<Material, i32> {
        let mut resource_delta: BTreeMap<Material, i32> = BTreeMap::new();
        for ScaledValidRecipe {
            valid_recipe,
            scale,
        } in self
            .cities
            .iter()
            .flat_map(|city| city.owned_buildings.iter())
            .filter(|owned_building| owned_building.owner_id == Some(business_id))
            .flat_map(|owned_building| owned_building.production_scale.iter())
        {
            for (material, quantity) in valid_recipe.get_recipe().materials {
                let scaled_quantity = quantity * scale.clone() as i32;
                if let Some(total_quantity) = resource_delta.get_mut(&material) {
                    *total_quantity += scaled_quantity;
                } else {
                    resource_delta.insert(material, scaled_quantity);
                }
            }
        }
        resource_delta
    }
    pub fn update_business_resources(&mut self, business_id: usize) {
        let resource_delta = self.get_resource_delta(business_id);
        let business = self.businesses.get_mut(business_id).unwrap();
        for (material, delta) in resource_delta {
            if let Some(quantity) = business.resources.get_mut(&material) {
                *quantity += delta;
            } else {
                business.resources.insert(material, delta);
            }
        }
    }
    fn get_net_products(&self, business_id: usize, resource: Material) -> i32 {
        self.cities
            .iter()
            .flat_map(|city| city.owned_buildings.iter())
            .filter(|owned_building| owned_building.owner_id == Some(business_id))
            .flat_map(|owned_building| {
                owned_building
                    .production_scale
                    .iter()
                    .map(|scaled_valid_recipe| {
                        *scaled_valid_recipe
                            .valid_recipe
                            .get_recipe()
                            .materials
                            .iter()
                            .find_map(|(material, count)| (material == &resource).then_some(count))
                            .unwrap_or(&0i32)
                            * (scaled_valid_recipe.scale as i32)
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
