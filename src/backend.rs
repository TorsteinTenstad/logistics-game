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

pub struct Building {
    pub building_type: BuildingType,
    pub production_scale: Vec<ScaledValidRecipe>,
    pub construction_cost: i32,
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
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
            construction_cost: match building_type {
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
}

#[derive(Debug, Clone)]
pub enum TerrainType {
    Grassland,
    Forrest,
    Desert,
    Hills,
    Mountain,
    Urban,
    WaterShallow,
    WaterDeep,
}

pub struct Tile {
    pub terrain_type: TerrainType,
    pub owner_id: Option<usize>,
    pub building: Option<Building>,
}

impl Tile {
    pub fn new(terrain_type: &TerrainType) -> Self {
        Self {
            terrain_type: terrain_type.clone(),
            owner_id: None,
            building: None,
        }
    }
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

pub struct GameData {
    pub tiles: Vec<Vec<Tile>>,
    pub businesses: Vec<Business>,
}

impl GameData {
    pub fn get_resource_stock(&self, business_id: usize) -> BTreeMap<Material, QuantityInfo> {
        let mut resource_stock: BTreeMap<Material, QuantityInfo> = BTreeMap::new();
        for ScaledValidRecipe {
            valid_recipe,
            scale,
            max_scale: _,
        } in self
            .tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|tile| {
                tile.building
                    .as_ref()
                    .filter(|_| tile.owner_id == Some(business_id))
            })
            .flat_map(|building| building.production_scale.iter())
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
