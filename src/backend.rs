use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Resource {
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

impl Resource {
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
    pub resources: Vec<(Resource, i32)>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ValidRecipe {
    ResourceImport(Resource),
    ResourceExport(Resource),
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
            resources: match self {
                Self::ResourceImport(resource) => {
                    let (quantity, price) = match resource {
                        Resource::Money => (1, 1),
                        Resource::Energy => (1, 2),
                        Resource::Sand => (2, 1),
                        Resource::Ore => (1, 4),
                        Resource::Gold => (1, 8),
                        Resource::Chip => (1, 1),
                        Resource::Wire => (1, 4),
                        Resource::Computer => (1, 8),
                        Resource::Log => (1, 2),
                        Resource::Plank => (1, 1),
                        Resource::Furniture => (1, 8),
                        Resource::RawOil => (1, 1),
                        Resource::Oil => (1, 2),
                        Resource::Glass => (1, 1),
                        Resource::Plastic => (1, 1),
                    };
                    vec![(Resource::Money, -price), (resource.clone(), quantity)]
                }
                Self::ResourceExport(resource) => {
                    let (quantity, price) = match resource {
                        Resource::Money => (1, 1),
                        Resource::Energy => (1, 2),
                        Resource::Sand => (2, 1),
                        Resource::Ore => (1, 4),
                        Resource::Gold => (1, 8),
                        Resource::Chip => (1, 1),
                        Resource::Wire => (1, 4),
                        Resource::Computer => (1, 8),
                        Resource::Log => (1, 2),
                        Resource::Plank => (1, 1),
                        Resource::Furniture => (1, 8),
                        Resource::RawOil => (1, 1),
                        Resource::Oil => (1, 2),
                        Resource::Glass => (1, 1),
                        Resource::Plastic => (1, 1),
                    };
                    vec![(resource.clone(), -quantity), (Resource::Money, price)]
                }
                Self::GlassProduction => vec![(Resource::Sand, -1), (Resource::Glass, 1)],
                Self::OilDrilling => vec![
                    (Resource::Money, -1),
                    (Resource::Energy, -1),
                    (Resource::RawOil, 4),
                ],
                Self::OilRefining => vec![(Resource::RawOil, -1), (Resource::Oil, 2)],
                Self::OilBurning => vec![(Resource::Oil, -1), (Resource::Energy, 2)],
                Self::ComputerAssembly => vec![
                    (Resource::Chip, -1),
                    (Resource::Wire, -1),
                    (Resource::Computer, 1),
                ],
                Self::PlankProduction => vec![(Resource::Log, -1), (Resource::Plank, 4)],
                Self::FurnitureProduction => vec![(Resource::Plank, -4), (Resource::Furniture, 1)],
                Self::OreMining => vec![(Resource::Energy, -1), (Resource::Ore, 1)],
                Self::MetalRefining => vec![
                    (Resource::Ore, -2),
                    (Resource::Wire, 1),
                    (Resource::Gold, 1),
                ],
                Self::SandCollecting => vec![(Resource::Energy, -1), (Resource::Sand, 2)],
                Self::Forestation => vec![(Resource::Energy, -1), (Resource::Log, 1)],
                Self::ChipProduction => vec![
                    (Resource::Energy, -1),
                    (Resource::Sand, -1),
                    (Resource::Chip, 1),
                ],
                Self::PlasticProduction => vec![(Resource::Oil, -1), (Resource::Plastic, 4)],
            },
        }
    }
}

pub struct ScaledValidRecipe {
    pub valid_recipe: ValidRecipe,
    pub scale: i32,
    pub max_scale: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildingType {
    LocalMarket,
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
    SolarEnergyFarm,
    WindEnergyFarm,
}

impl BuildingType {
    fn get_valid_recipes(&self) -> Vec<ValidRecipe> {
        match self {
            Self::LocalMarket => {
                vec![
                    ValidRecipe::ResourceExport(Resource::Glass),
                    ValidRecipe::ResourceImport(Resource::Wire),
                    ValidRecipe::ResourceExport(Resource::Wire),
                    ValidRecipe::ResourceExport(Resource::Chip),
                    ValidRecipe::ResourceExport(Resource::Gold),
                    ValidRecipe::ResourceExport(Resource::Ore),
                    ValidRecipe::ResourceImport(Resource::Log),
                    ValidRecipe::ResourceExport(Resource::Log),
                    ValidRecipe::ResourceExport(Resource::Plastic),
                    ValidRecipe::ResourceExport(Resource::Computer),
                ]
            }
            Self::EnergyMarket => {
                vec![
                    ValidRecipe::ResourceImport(Resource::Energy),
                    ValidRecipe::ResourceExport(Resource::Energy),
                ]
            }
            Self::WoodWorkingMarket => {
                vec![
                    ValidRecipe::ResourceImport(Resource::Plank),
                    ValidRecipe::ResourceExport(Resource::Plank),
                    ValidRecipe::ResourceExport(Resource::Furniture),
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
            Self::SolarEnergyFarm => {
                vec![ValidRecipe::ResourceExport(Resource::Energy)]
            }
            Self::WindEnergyFarm => {
                vec![ValidRecipe::ResourceExport(Resource::Energy)]
            }
        }
    }
    pub fn get_construction_cost(&self) -> i32 {
        match self {
            BuildingType::LocalMarket => 20,
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
            BuildingType::SolarEnergyFarm => 200,
            BuildingType::WindEnergyFarm => 200,
        }
    }
}

pub struct Building {
    pub building_type: BuildingType,
    pub production_scale: Vec<ScaledValidRecipe>,
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
    Industrial,
    WaterShallow,
    WaterDeep,
}

impl TerrainType {
    pub fn supported_building_types(&self) -> Vec<BuildingType> {
        match self {
            Self::Grassland => vec![BuildingType::SolarEnergyFarm, BuildingType::WindEnergyFarm],
            Self::Forrest => vec![BuildingType::TreeFarm],
            Self::Desert => vec![
                BuildingType::SolarEnergyFarm,
                BuildingType::SandPlant,
                BuildingType::OilRig,
            ],
            Self::Hills => vec![BuildingType::OilRig],
            Self::Mountain => vec![],
            Self::Urban => vec![BuildingType::LocalMarket],
            Self::Industrial => vec![
                BuildingType::EnergyMarket,
                BuildingType::FurnitureFactory,
                BuildingType::MetalRefinery,
                BuildingType::GlassFactory,
                BuildingType::OilRefinery,
                BuildingType::Sawmill,
                BuildingType::PlasticFactory,
                BuildingType::OilEnergyPlant,
                BuildingType::WoodWorkingMarket,
            ],
            Self::WaterShallow => vec![],
            Self::WaterDeep => vec![],
        }
    }
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
    pub fn get_acquisition_cost(&self) -> i32 {
        100
    }
}

#[derive(Default)]
pub struct Business {
    pub resources: BTreeMap<Resource, i32>,
}

impl Business {
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::from([(Resource::Money, 250)]),
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
    pub fn get_resource_stock(&self, business_id: usize) -> BTreeMap<Resource, QuantityInfo> {
        let mut resource_stock: BTreeMap<Resource, QuantityInfo> = BTreeMap::new();
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
            for (resource, quantity) in valid_recipe.get_recipe().resources {
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
                if let Some(quantity_info) = resource_stock.get_mut(&resource) {
                    quantity_info.gross_in += gross_in;
                    quantity_info.gross_out += gross_out;
                } else {
                    resource_stock.insert(
                        resource.clone(),
                        QuantityInfo {
                            quantity: self
                                .businesses
                                .get(business_id)
                                .unwrap()
                                .resources
                                .get(&resource)
                                .unwrap_or(&0)
                                .clone(),
                            gross_in: gross_in,
                            gross_out: gross_out,
                        },
                    );
                }
            }
        }
        for (resource, quantity) in self.businesses.get(business_id).unwrap().resources.iter() {
            if resource_stock.contains_key(resource) {
                continue;
            }
            resource_stock.insert(
                resource.clone(),
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
        for (resource, quantity_info) in resource_stock {
            let delta = quantity_info.gross_in - quantity_info.gross_out;
            if let Some(quantity) = business.resources.get_mut(&resource) {
                *quantity += delta;
            } else {
                business.resources.insert(resource, delta);
            }
        }
    }
}
