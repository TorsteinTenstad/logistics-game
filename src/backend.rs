use rand::{rngs::ThreadRng, Rng};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Material {
    Money,
    Energy,
    Sand,
    Ore,
    Gold,
    Chip,
    Wire,
    Computer,
    Log,
    Plank,
    Furniture,
    RawOil,
    Oil,
    Glass,
    Plastic,
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
            Self::Wire => "wire",
            Self::Computer => "computer",
            Self::Log => "logs",
            Self::Plank => "planks",
            Self::Furniture => "chair",
            Self::RawOil => "raw_oil",
            Self::Oil => "oil",
            Self::Glass => "glass",
            Self::Plastic => "plastic",
        }
        .to_string()
    }
}

pub struct Recipe {
    pub materials: Vec<(Material, i32)>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ValidRecipe {
    MaterialImport(Material),
    MaterialExport(Material),
    ComputerAssembly,
    PlankProduction,
    FurnitureProduction,
    OreMining,
    MetalRefining,
    SandCollecting,
    ChipProduction,
    GlassProduction,
    OilDrilling,
    OilRefining,
    OilBurning,
    PlasticProduction,
    Forestation,
}

impl ValidRecipe {
    pub fn get_recipe(&self) -> Recipe {
        Recipe {
            materials: match self {
                Self::MaterialImport(material) => {
                    let (quantity, price) = match material {
                        Material::Money => (1, 1),
                        Material::Energy => (1, 2),
                        Material::Sand => (2, 1),
                        Material::Ore => (1, 4),
                        Material::Gold => (1, 8),
                        Material::Chip => (1, 1),
                        Material::Wire => (1, 4),
                        Material::Computer => (1, 8),
                        Material::Log => (1, 2),
                        Material::Plank => (1, 1),
                        Material::Furniture => (1, 8),
                        Material::RawOil => (1, 1),
                        Material::Oil => (1, 2),
                        Material::Glass => (1, 1),
                        Material::Plastic => (1, 1),
                    };
                    vec![(Material::Money, -price), (material.clone(), quantity)]
                }
                Self::MaterialExport(material) => {
                    let (quantity, price) = match material {
                        Material::Money => (1, 1),
                        Material::Energy => (1, 2),
                        Material::Sand => (2, 1),
                        Material::Ore => (1, 4),
                        Material::Gold => (1, 8),
                        Material::Chip => (1, 1),
                        Material::Wire => (1, 4),
                        Material::Computer => (1, 8),
                        Material::Log => (1, 2),
                        Material::Plank => (1, 1),
                        Material::Furniture => (1, 8),
                        Material::RawOil => (1, 1),
                        Material::Oil => (1, 2),
                        Material::Glass => (1, 1),
                        Material::Plastic => (1, 1),
                    };
                    vec![(material.clone(), -quantity), (Material::Money, price)]
                }
                Self::GlassProduction => vec![(Material::Sand, -1), (Material::Glass, 1)],
                Self::OilDrilling => vec![
                    (Material::Money, -1),
                    (Material::Energy, -1),
                    (Material::RawOil, 4),
                ],
                Self::OilRefining => vec![(Material::RawOil, -1), (Material::Oil, 2)],
                Self::OilBurning => vec![(Material::Oil, -1), (Material::Energy, 2)],
                Self::ComputerAssembly => vec![
                    (Material::Chip, -1),
                    (Material::Wire, -1),
                    (Material::Computer, 1),
                ],
                Self::PlankProduction => vec![(Material::Log, -1), (Material::Plank, 4)],
                Self::FurnitureProduction => vec![(Material::Plank, -4), (Material::Furniture, 1)],
                Self::OreMining => vec![(Material::Energy, -1), (Material::Ore, 1)],
                Self::MetalRefining => vec![
                    (Material::Ore, -2),
                    (Material::Wire, 1),
                    (Material::Gold, 1),
                ],
                Self::SandCollecting => vec![(Material::Energy, -1), (Material::Sand, 2)],
                Self::Forestation => vec![(Material::Energy, -1), (Material::Log, 1)],
                Self::ChipProduction => vec![
                    (Material::Energy, -1),
                    (Material::Sand, -1),
                    (Material::Chip, 1),
                ],
                Self::PlasticProduction => vec![(Material::Oil, -1), (Material::Plastic, 4)],
            },
        }
    }
}

pub struct ScaledValidRecipe {
    pub valid_recipe: ValidRecipe,
    pub scale: i32,
    pub max_scale: i32,
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    Market,
    EnergyMarket,
    Sawmill,
    FurnitureFactory,
    WoodWorkingMarket,
    ComputerFactory,
    SandPlant,
    Mine,
    MetalRefinery,
    GlassFactory,
    OilRig,
    OilRefinery,
    PlasticFactory,
    OilEnergyPlant,
    TreeFarm,
}

impl BuildingType {
    fn get_valid_recipes(&self) -> Vec<ValidRecipe> {
        match self {
            Self::Market => {
                vec![
                    ValidRecipe::MaterialExport(Material::Glass),
                    ValidRecipe::MaterialImport(Material::Wire),
                    ValidRecipe::MaterialExport(Material::Wire),
                    ValidRecipe::MaterialExport(Material::Chip),
                    ValidRecipe::MaterialExport(Material::Gold),
                    ValidRecipe::MaterialExport(Material::Ore),
                    ValidRecipe::MaterialImport(Material::Log),
                    ValidRecipe::MaterialExport(Material::Log),
                    ValidRecipe::MaterialExport(Material::Plastic),
                    ValidRecipe::MaterialExport(Material::Computer),
                ]
            }
            Self::EnergyMarket => {
                vec![
                    ValidRecipe::MaterialImport(Material::Energy),
                    ValidRecipe::MaterialExport(Material::Energy),
                ]
            }
            Self::WoodWorkingMarket => {
                vec![
                    ValidRecipe::MaterialImport(Material::Plank),
                    ValidRecipe::MaterialExport(Material::Plank),
                    ValidRecipe::MaterialExport(Material::Furniture),
                ]
            }
            Self::ComputerFactory => {
                vec![ValidRecipe::ChipProduction, ValidRecipe::ComputerAssembly]
            }
            Self::TreeFarm => vec![ValidRecipe::Forestation],
            Self::Sawmill => vec![ValidRecipe::PlankProduction],
            Self::FurnitureFactory => vec![ValidRecipe::FurnitureProduction],
            Self::SandPlant => vec![ValidRecipe::SandCollecting],
            Self::GlassFactory => vec![ValidRecipe::GlassProduction],
            Self::Mine => vec![ValidRecipe::OreMining],
            Self::MetalRefinery => vec![ValidRecipe::MetalRefining],
            Self::OilRig => vec![ValidRecipe::OilDrilling],
            Self::OilRefinery => vec![ValidRecipe::OilRefining],
            Self::OilEnergyPlant => vec![ValidRecipe::OilBurning],
            Self::PlasticFactory => vec![ValidRecipe::PlasticProduction],
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
                    max_scale: 5,
                })
                .collect(),
            owner_id: None,
            acquisition_cost: match building_type {
                BuildingType::Market => 20,
                BuildingType::EnergyMarket => 20,
                BuildingType::Sawmill => 80,
                BuildingType::FurnitureFactory => 60,
                BuildingType::WoodWorkingMarket => 20,
                BuildingType::ComputerFactory => 150,
                BuildingType::TreeFarm => 50,
                BuildingType::SandPlant => 80,
                BuildingType::Mine => 100,
                BuildingType::MetalRefinery => 100,
                BuildingType::GlassFactory => 50,
                BuildingType::OilRig => 100,
                BuildingType::OilRefinery => 100,
                BuildingType::PlasticFactory => 100,
                BuildingType::OilEnergyPlant => 100,
            },
        }
    }
    pub fn new_random(rng: &mut ThreadRng) -> Self {
        let options = vec![
            BuildingType::Market,
            BuildingType::EnergyMarket,
            BuildingType::Sawmill,
            BuildingType::FurnitureFactory,
            BuildingType::WoodWorkingMarket,
            BuildingType::ComputerFactory,
            BuildingType::SandPlant,
            BuildingType::TreeFarm,
            BuildingType::Mine,
            BuildingType::MetalRefinery,
            BuildingType::GlassFactory,
            BuildingType::OilRig,
            BuildingType::OilRefinery,
            BuildingType::OilEnergyPlant,
            BuildingType::PlasticFactory,
        ];
        Self::new(
            options
                .get(rng.gen_range(0..options.len()))
                .unwrap()
                .clone(),
        )
    }
}

pub struct City {
    pub x: f32,
    pub y: f32,
    pub owned_buildings: Vec<OwnedBuilding>,
}

impl City {
    pub fn new_with_random_buildings(rng: &mut ThreadRng, x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
            owned_buildings: (0..rng.gen_range(1..7))
                .into_iter()
                .map(|_i| OwnedBuilding::new_random(rng))
                .collect(),
        }
    }
}

pub struct OwnedConnection {
    pub city_ids: Vec<usize>,
    pub owner_id: Option<usize>,
    pub acquisition_cost: i32,
}

impl OwnedConnection {
    pub fn new(city_id_a: usize, city_id_b: usize) -> Self {
        Self {
            city_ids: vec![city_id_a, city_id_b],
            owner_id: None,
            acquisition_cost: 20,
        }
    }
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
            resources: BTreeMap::from([(Material::Money, 250)]),
        }
    }
}

pub struct QuantityInfo {
    pub quantity: i32,
    pub gross_in: i32,
    pub gross_out: i32,
}

impl QuantityInfo {
    pub fn net_in(&self) -> i32 {
        self.gross_in - self.gross_out
    }
    pub fn net_out(&self) -> i32 {
        self.gross_out - self.gross_in
    }
}

impl Graph {
    pub fn get_resource_stock(&self, business_id: usize) -> BTreeMap<Material, QuantityInfo> {
        let mut resource_stock: BTreeMap<Material, QuantityInfo> = BTreeMap::new();
        for ScaledValidRecipe {
            valid_recipe,
            scale,
            max_scale: _,
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
