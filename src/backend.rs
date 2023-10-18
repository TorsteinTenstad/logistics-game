use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Material {
    Money,
    Energy,
    Sand,
    Ore,
    Gold,
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
            Self::Money => "money",
            Self::Energy => "energy",
            Self::Sand => "sand",
            Self::Gold => "gold",
            Self::Ore => "rocks",
            Self::Chip => "chip",
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
    OreMining,
    MetalRefining,
    SandCollecting,
    ChipProduction,
    LogImport,
    FurnitureExport,
    GoldExport,
    ComputerExport,
}

impl ValidRecipe {
    pub fn get_recipe(&self) -> Recipe {
        Recipe {
            materials: match self {
                Self::ComputerAssembly => vec![
                    (Material::Chip, -1),
                    (Material::Wire, -1),
                    (Material::Computer, 1),
                ],
                Self::PlankProduction => vec![(Material::Log, -1), (Material::Plank, 1)],
                Self::FurnitureProduction => vec![(Material::Plank, -1), (Material::Furniture, 1)],
                Self::OreMining => vec![(Material::Energy, -1), (Material::Ore, 1)],
                Self::MetalRefining => vec![
                    (Material::Ore, -1),
                    (Material::Wire, 1),
                    (Material::Gold, 1),
                ],
                Self::SandCollecting => vec![(Material::Energy, -1), (Material::Sand, 3)],
                Self::ChipProduction => vec![
                    (Material::Energy, -1),
                    (Material::Sand, -1),
                    (Material::Chip, 1),
                ],
                Self::LogImport => vec![(Material::Money, -10), (Material::Log, 1)],
                Self::FurnitureExport => vec![(Material::Furniture, -1), (Material::Money, 100)],
                Self::GoldExport => vec![(Material::Gold, -1), (Material::Money, 100)],
                Self::ComputerExport => vec![(Material::Computer, -1), (Material::Money, 100)],
            },
        }
    }
}

pub struct ScaledValidRecipe {
    pub valid_recipe: ValidRecipe,
    pub scale: i32,
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    Market,
    WoodWorkingFactory,
    ComputerFactory,
    SandPlant,
    Mine,
    MetalRefinery,
}

impl BuildingType {
    fn get_valid_recipes(&self) -> Vec<ValidRecipe> {
        match self {
            Self::Market => {
                vec![
                    ValidRecipe::LogImport,
                    ValidRecipe::FurnitureExport,
                    ValidRecipe::GoldExport,
                    ValidRecipe::ComputerExport,
                ]
            }
            Self::ComputerFactory => {
                vec![ValidRecipe::ChipProduction, ValidRecipe::ComputerAssembly]
            }
            Self::WoodWorkingFactory => vec![
                ValidRecipe::PlankProduction,
                ValidRecipe::FurnitureProduction,
            ],
            Self::SandPlant => vec![ValidRecipe::SandCollecting],
            Self::Mine => vec![ValidRecipe::OreMining],
            Self::MetalRefinery => vec![ValidRecipe::MetalRefining],
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
            resources: BTreeMap::from([(Material::Money, 1000)]),
        }
    }
}

pub struct QuantityInfo {
    pub quantity: i32,
    pub gross_in: i32,
    pub gross_out: i32,
}

impl Graph {
    pub fn get_resource_stock(&self, business_id: usize) -> BTreeMap<Material, QuantityInfo> {
        let mut resource_stock: BTreeMap<Material, QuantityInfo> = BTreeMap::new();
        for ScaledValidRecipe {
            valid_recipe,
            scale,
        } in self
            .cities
            .iter()
            .flat_map(|city| city.owned_buildings.iter())
            .filter(|owned_building| owned_building.owner_id == Some(business_id))
            .flat_map(|owned_building| owned_building.production_scale.iter())
            .filter(|s| (s.scale != 0))
        {
            for (material, quantity) in valid_recipe.get_recipe().materials {
                let scaled_quantity = quantity * scale.clone() as i32;
                let gross_in = if scaled_quantity > 0 {
                    scaled_quantity
                } else {
                    0
                };
                let gross_out = if scaled_quantity < 0 {
                    -scaled_quantity
                } else {
                    0
                };
                if let Some(quantity_info) = resource_stock.get_mut(&material) {
                    quantity_info.gross_in += gross_in;
                    quantity_info.gross_out += gross_out;
                } else {
                    resource_stock.insert(
                        material.clone(),
                        QuantityInfo {
                            quantity: self
                                .businesses
                                .get(business_id)
                                .unwrap()
                                .resources
                                .get(&material)
                                .unwrap_or(&0)
                                .clone(),
                            gross_in: gross_in,
                            gross_out: gross_out,
                        },
                    );
                }
            }
        }
        for (material, quantity) in self.businesses.get(business_id).unwrap().resources.iter() {
            if resource_stock.contains_key(material) {
                continue;
            }
            resource_stock.insert(
                material.clone(),
                QuantityInfo {
                    quantity: quantity.clone(),
                    gross_in: 0,
                    gross_out: 0,
                },
            );
        }
        resource_stock
    }
    pub fn update_business_resources(&mut self, business_id: usize) {
        let resource_stock = self.get_resource_stock(business_id);
        let business = self.businesses.get_mut(business_id).unwrap();
        for (material, quantity_info) in resource_stock {
            let delta = quantity_info.gross_in - quantity_info.gross_out;
            if let Some(quantity) = business.resources.get_mut(&material) {
                *quantity += delta;
            } else {
                business.resources.insert(material, delta);
            }
        }
    }
}
