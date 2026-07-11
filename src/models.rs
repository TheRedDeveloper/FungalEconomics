use ply_engine::prelude::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, BitOr, BitOrAssign};

pub static UNDO_IMAGE: GraphicAsset = GraphicAsset::Bytes { file_name: "undo.png", data: include_bytes!("../assets/images/undo.png") };
pub static STACK_IMAGE: GraphicAsset = GraphicAsset::Bytes { file_name: "stack.png", data: include_bytes!("../assets/images/stack.png") };

// Tiles
static ASH_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "AshWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Ash.png") };
static CHARREDFALLENLOG_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredFallenLogWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/CharredFallenLog.png") };
static CHARREDTREETRUNK_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredTreeTrunkWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/CharredTreeTrunk.png") };
static CHARREDGRASS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredGrassWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/CharredGrass.png") };
static PUDDLE_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "PuddleWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Puddle.png") };
static DRYDIRT_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "DryDirtWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/DryDirt.png") };
static DIRT_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "DirtWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Dirt.png") };
static GRASS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "GrassWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Grass.png") };
static SHRUB_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "ShrubWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Shrub.png") };
static FLOWERS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "FlowersWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Flowers.png") };
static SAPLINGS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "SaplingsWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Saplings.png") };
static MOSS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "MossWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Moss.png") };
static CLOVER_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CloverWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Clover.png") };
static BERRYBUSH_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "BerryBushWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/BerryBush.png") };
static FERNS_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "FernsWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Ferns.png") };
static BIRCH_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "BirchWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Birch.png") };
static PINE_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "PineWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Pine.png") };
static OAK_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "OakWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/Oak.png") };
static GREENPUDDLE_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "GreenPuddleWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/GreenPuddle.png") };
static COARSEDIRT_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CoarseDirtWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/CoarseDirt.png") };
static LEAFLITTER_TILE_WITH_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "LeafLitterWithYield.png", data: include_bytes!("../assets/images/tiles/withYield/LeafLitter.png") };

static ASH_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "AshWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Ash.png") };
static CHARREDFALLENLOG_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredFallenLogWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/CharredFallenLog.png") };
static CHARREDTREETRUNK_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredTreeTrunkWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/CharredTreeTrunk.png") };
static CHARREDGRASS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CharredGrassWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/CharredGrass.png") };
static PUDDLE_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "PuddleWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Puddle.png") };
static DRYDIRT_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "DryDirtWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/DryDirt.png") };
static DIRT_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "DirtWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Dirt.png") };
static GRASS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "GrassWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Grass.png") };
static SHRUB_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "ShrubWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Shrub.png") };
static FLOWERS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "FlowersWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Flowers.png") };
static SAPLINGS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "SaplingsWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Saplings.png") };
static MOSS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "MossWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Moss.png") };
static CLOVER_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CloverWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Clover.png") };
static BERRYBUSH_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "BerryBushWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/BerryBush.png") };
static FERNS_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "FernsWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Ferns.png") };
static BIRCH_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "BirchWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Birch.png") };
static PINE_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "PineWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Pine.png") };
static OAK_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "OakWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/Oak.png") };
static GREENPUDDLE_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "GreenPuddleWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/GreenPuddle.png") };
static COARSEDIRT_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "CoarseDirtWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/CoarseDirt.png") };
static LEAFLITTER_TILE_WITHOUT_YIELD: GraphicAsset = GraphicAsset::Bytes { file_name: "LeafLitterWithoutYield.png", data: include_bytes!("../assets/images/tiles/withoutYield/LeafLitter.png") };

#[derive(Clone, Copy, Debug, Default)]
pub struct Resources {
  pub carbon: f32,
  pub nitrogen: f32,
  pub phosphorus: f32,
  pub water: f32,
}

impl Resources {
  pub fn new(c: f32, n: f32, p: f32, w: f32) -> Self {
    Self { carbon: c, nitrogen: n, phosphorus: p, water: w }
  }

  pub fn minimum_fraction_fulfilled(&self, cost: &Resources) -> (f32, IsResourceMissing) {
    let mut missing = IsResourceMissing::default();
    let mut min_frac = 1.0f32;

    if cost.carbon > 0.0 {
      let frac = (self.carbon / cost.carbon).min(1.0);
      if frac < 1.0 { missing.carbon = true; }
      min_frac = min_frac.min(frac);
    }
    if cost.nitrogen > 0.0 {
      let frac = (self.nitrogen / cost.nitrogen).min(1.0);
      if frac < 1.0 { missing.nitrogen = true; }
      min_frac = min_frac.min(frac);
    }
    if cost.phosphorus > 0.0 {
      let frac = (self.phosphorus / cost.phosphorus).min(1.0);
      if frac < 1.0 { missing.phosphorus = true; }
      min_frac = min_frac.min(frac);
    }
    if cost.water > 0.0 {
      let frac = (self.water / cost.water).min(1.0);
      if frac < 1.0 { missing.water = true; }
      min_frac = min_frac.min(frac);
    }

    (min_frac.max(0.0), missing)
  }
}

impl Add for Resources {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      carbon: self.carbon + other.carbon,
      nitrogen: self.nitrogen + other.nitrogen,
      phosphorus: self.phosphorus + other.phosphorus,
      water: self.water + other.water,
    }
  }
}

impl AddAssign for Resources {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other;
  }
}

impl Sub for Resources {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      carbon: (self.carbon - other.carbon).max(0.0),
      nitrogen: (self.nitrogen - other.nitrogen).max(0.0),
      phosphorus: (self.phosphorus - other.phosphorus).max(0.0),
      water: (self.water - other.water).max(0.0),
    }
  }
}

impl SubAssign for Resources {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl Mul<f32> for Resources {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    Self {
      carbon: self.carbon * rhs,
      nitrogen: self.nitrogen * rhs,
      phosphorus: self.phosphorus * rhs,
      water: self.water * rhs,
    }
  }
}

impl Div<f32> for Resources {
  type Output = Self;
  fn div(self, rhs: f32) -> Self {
    Self {
      carbon: self.carbon / rhs,
      nitrogen: self.nitrogen / rhs,
      phosphorus: self.phosphorus / rhs,
      water: self.water / rhs,
    }
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IsResourceMissing {
  pub carbon: bool,
  pub nitrogen: bool,
  pub phosphorus: bool,
  pub water: bool,
}

impl BitOr for IsResourceMissing {
  type Output = Self;
  fn bitor(self, other: Self) -> Self {
    Self {
      carbon: self.carbon | other.carbon,
      nitrogen: self.nitrogen | other.nitrogen,
      phosphorus: self.phosphorus | other.phosphorus,
      water: self.water | other.water,
    }
  }
}

// In rust BitOr is |
impl BitOrAssign for IsResourceMissing {
  fn bitor_assign(&mut self, other: Self) {
    self.carbon |= other.carbon;
    self.nitrogen |= other.nitrogen;
    self.phosphorus |= other.phosphorus;
    self.water |= other.water;
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseTileType {
  Ash,
  CharredFallenLog,
  CharredTreeTrunk,
  CharredGrass,
  Puddle,
  DryDirt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
  Ash,
  CharredFallenLog,
  CharredTreeTrunk,
  CharredGrass,
  Puddle,
  DryDirt,
  Dirt,
  Grass,
  Shrub,
  Flowers,
  Saplings,
  Moss,
  Clover,
  BerryBush,
  Ferns,
  Birch,
  Pine,
  Oak,
  GreenPuddle,
  CoarseDirt,
  LeafLitter,
}

pub struct Trade {
  pub consumes_per_tick: Option<Resources>,
  pub yields_per_tick: Resources,
}

#[derive(Clone, Debug)]
pub struct ButtonData {
  pub is_investing: bool,
  pub amount: Resources,
  pub fraction: f32,
}

#[derive(Clone, Debug)]
pub enum Change {
  Overtake(BaseTileType),
  Add(BaseTileType),
  Spore,
  Stack(BaseTileType),
}
impl Change {
  pub fn label(&self) -> String {
    match self {
      Change::Overtake(_) => "ÜBERNEHMEN".to_string(),
      Change::Add(_) => "KAUFEN".to_string(),
      Change::Spore => "SPOREN".to_string(),
      Change::Stack(_) => "STAPELN".to_string(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct GameState {
  pub is_paused: bool,
  pub current_phase: u8, // 1 to 5
  pub phase_timer: f32,
  pub resource_pool: Resources,
  pub is_resource_missing: IsResourceMissing,
  pub overstacked_menu: Option<Option<BaseTileType>>,
  pub active_nodes: Vec<BaseTileType>,
  pub spore_points: u32,
  pub invest_button_data: Vec<ButtonData>, // last is always "spore" button, rest are for node buying
  pub income_per_tick: Resources,
  pub change_log: Vec<Change>,
  pub stack_mode: bool,
}
impl GameState {
  pub fn new() -> Self {
    let mut new = Self {
      is_paused: false,
      current_phase: 1,
      phase_timer: PHASE_LENGTH,
      resource_pool: Resources::new(START_CARBON, START_NITROGEN, START_PHOSPHORUS, START_WATER),
      is_resource_missing: IsResourceMissing::default(),
      overstacked_menu: None,
      active_nodes: vec![BaseTileType::Puddle], // Free starting token
      spore_points: 0,
      invest_button_data: vec![],
      income_per_tick: BASE_INCOME,
      change_log: vec![],
      stack_mode: false,
    };
    new.reset_button_data();
    new
  }

  pub fn reset_button_data(&mut self) {
    self.invest_button_data.clear();
    for base in BaseTileType::base_types_by_phase(self.current_phase) {
      let tile = base.get_current_tile_type(self.current_phase);
      let cost = Resources {
        carbon: tile.expansion_carbon_cost(),
        water: tile.water_cost(),
        nitrogen: 0.0,
        phosphorus: 0.0,
      };
      self.invest_button_data.push(ButtonData {
        is_investing: false,
        amount: cost,
        fraction: 0.0,
      });
    }
    self.invest_button_data.push(ButtonData {
      is_investing: false,
      amount: SPORE_POINT_COSTS,
      fraction: 0.0,
    });
  }

  pub fn undo_last_change(&mut self) {
    if let Some(change) = self.change_log.pop() {
      match change {
        Change::Overtake(base) => {
          self.active_nodes.push(base);
        }
        Change::Add(base) => {
          if let Some(pos) = self.active_nodes.iter().position(|&b| b == base) {
            self.active_nodes.remove(pos);
            let tile = base.get_current_tile_type(self.current_phase);
            self.resource_pool.carbon += tile.expansion_carbon_cost();
            self.resource_pool.water += tile.water_cost();
          } else {
            eprintln!("Warning: Tried to undo Add for base {:?}, but it was not found in active_nodes.", base);
          }
        }
        Change::Spore => {
          if self.spore_points > 0 {
            self.spore_points -= 1;
            self.resource_pool += SPORE_POINT_COSTS;
          } else {
            eprintln!("Warning: Tried to undo Spore change, but spore_points is already 0.");
          }
        }
        Change::Stack(base) => {
          let tile = base.get_current_tile_type(self.current_phase);
          self.resource_pool.carbon += tile.expansion_carbon_cost();
          self.resource_pool.water += tile.water_cost();
        }
      }
    }
  }
}

#[derive(Clone, Debug)]
pub enum GameMode {
  StartSync { hold_accumulation: f32 },
  Playing { state: GameState },
  TransitionSync { state: GameState, hold_accumulator: f32 },
  GameOver { state: GameState },
}

pub const SPORE_POINT_COSTS: Resources = Resources {
  carbon: 500.0,
  nitrogen: 50.0,
  phosphorus: 50.0,
  water: 50.0,
};

pub const TICK_LENGTH: f32 = 5.0;
pub const PHASE_LENGTH: f32 = 120.0;
pub const START_CARBON: f32 = 200.0;
pub const START_NITROGEN: f32 = 50.0;
pub const START_PHOSPHORUS: f32 = 50.0;
pub const START_WATER: f32 = 50.0;
pub const SYNC_HOLD_TIME: f32 = 10.0;
pub const DRAIN_TIME: f32 = 1.0;

pub const BASE_INCOME: Resources = Resources {
  carbon: 5.0,
  nitrogen: 5.0,
  phosphorus: 5.0,
  water: 5.0,
};

impl BaseTileType {
  pub fn get_current_tile_type(&self, phase: u8) -> TileType {
    match self {
      BaseTileType::Ash => match phase {
        1 => TileType::Ash,
        2 => TileType::Dirt,
        3 => TileType::Moss,
        4 => TileType::Pine,
        _ => TileType::Oak,
      },
      BaseTileType::CharredFallenLog => match phase {
        1 => TileType::CharredFallenLog,
        2 => TileType::Grass,
        3 => TileType::Clover,
        4 => TileType::Pine,
        _ => TileType::Oak,
      },
      BaseTileType::CharredTreeTrunk => match phase {
        1 => TileType::CharredTreeTrunk,
        2 => TileType::Shrub,
        3 => TileType::BerryBush,
        4 => TileType::Oak,
        _ => TileType::Oak,
      },
      BaseTileType::CharredGrass => match phase {
        1 => TileType::CharredGrass,
        2 => TileType::Flowers,
        3 => TileType::Ferns,
        4 => TileType::Oak,
        _ => TileType::Oak,
      },
      BaseTileType::Puddle => match phase {
        1 | 2 | 3 => TileType::Puddle,
        4 => TileType::GreenPuddle,
        _ => TileType::LeafLitter,
      },
      BaseTileType::DryDirt => match phase {
        1 => TileType::DryDirt,
        2 => TileType::Saplings,
        3 => TileType::Birch,
        4 => TileType::CoarseDirt,
        _ => TileType::LeafLitter,
      },
    }
  }

  pub fn base_types_by_phase(phase: u8) -> Vec<BaseTileType> {
    match phase {
      1 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
      2 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
      3 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
      4 => vec![BaseTileType::Ash, BaseTileType::CharredTreeTrunk, BaseTileType::Puddle, BaseTileType::DryDirt],
      5 => vec![BaseTileType::Ash, BaseTileType::Puddle],
      _ => vec![],
    }
  }
}

impl TileType {
  pub fn water_cost(&self) -> f32 {
    match self {
      TileType::Puddle | TileType::CharredFallenLog | TileType::Ferns | TileType::LeafLitter | TileType::GreenPuddle => 10.0,
      TileType::Ash | TileType::CharredGrass | TileType::DryDirt | TileType::Dirt | TileType::Grass | TileType::Shrub | TileType::Flowers | TileType::Saplings | TileType::Moss | TileType::Clover | TileType::BerryBush | TileType::CoarseDirt => 25.0,
      TileType::CharredTreeTrunk | TileType::Birch | TileType::Pine | TileType::Oak => 50.0,
    }
  }

  pub fn expansion_carbon_cost(&self) -> f32 {
    200.0
  }

  pub fn get_trade(&self) -> Trade {
    match self {
      TileType::Ash => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(20.0, 3.0, 5.0, 0.0),
      },
      TileType::CharredFallenLog => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 2.0, 0.0, 1.0)),
        yields_per_tick: Resources::new(20.0, 0.0, 0.0, 0.0),
      },
      TileType::CharredTreeTrunk => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 4.0, 2.0)),
        yields_per_tick: Resources::new(25.0, 0.0, 0.0, 0.0),
      },
      TileType::CharredGrass => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(12.0, 4.0, 10.0, 0.0),
      },
      TileType::Puddle => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 0.0, 0.0, 15.0),
      },
      TileType::DryDirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(10.0, 6.0, 4.0, 4.0),
      },
      TileType::Dirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 20.0, 20.0, 2.0),
      },
      TileType::Grass => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 3.0, 0.0, 3.0)),
        yields_per_tick: Resources::new(20.0, 0.0, 0.0, 0.0),
      },
      TileType::Shrub => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 4.0, 3.0)),
        yields_per_tick: Resources::new(30.0, 0.0, 0.0, 0.0),
      },
      TileType::Flowers => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 2.0, 2.0, 0.0)),
        yields_per_tick: Resources::new(30.0, 0.0, 0.0, 4.0),
      },
      TileType::Saplings => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 4.0, 2.0, 4.0)),
        yields_per_tick: Resources::new(40.0, 0.0, 0.0, 0.0),
      },
      TileType::Moss => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(12.0, 15.0, 10.0, 8.0),
      },
      TileType::Clover => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 2.0, 2.0)),
        yields_per_tick: Resources::new(20.0, 12.0, 0.0, 0.0),
      },
      TileType::BerryBush => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 2.0, 6.0, 2.0)),
        yields_per_tick: Resources::new(35.0, 0.0, 0.0, 0.0),
      },
      TileType::Ferns => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 0.0, 2.0)),
        yields_per_tick: Resources::new(15.0, 8.0, 12.0, 0.0),
      },
      TileType::Birch => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 10.0, 8.0, 6.0)),
        yields_per_tick: Resources::new(45.0, 0.0, 0.0, 0.0),
      },
      TileType::Pine => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 15.0, 10.0, 8.0)),
        yields_per_tick: Resources::new(50.0, 0.0, 0.0, 0.0),
      },
      TileType::Oak => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 20.0, 20.0, 15.0)),
        yields_per_tick: Resources::new(100.0, 0.0, 0.0, 0.0),
      },
      TileType::GreenPuddle => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(20.0, 30.0, 20.0, 20.0),
      },
      TileType::CoarseDirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(2.0, 15.0, 20.0, 2.0),
      },
      TileType::LeafLitter => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(5.0, 20.0, 20.0, 8.0),
      },
    }
  }

  pub fn graphic_with_yield(&self) -> &'static GraphicAsset {
    match self {
      TileType::Ash => &ASH_TILE_WITH_YIELD,
      TileType::CharredFallenLog => &CHARREDFALLENLOG_TILE_WITH_YIELD,
      TileType::CharredTreeTrunk => &CHARREDTREETRUNK_TILE_WITH_YIELD,
      TileType::CharredGrass => &CHARREDGRASS_TILE_WITH_YIELD,
      TileType::Puddle => &PUDDLE_TILE_WITH_YIELD,
      TileType::DryDirt => &DRYDIRT_TILE_WITH_YIELD,
      TileType::Dirt => &DIRT_TILE_WITH_YIELD,
      TileType::Grass => &GRASS_TILE_WITH_YIELD,
      TileType::Shrub => &SHRUB_TILE_WITH_YIELD,
      TileType::Flowers => &FLOWERS_TILE_WITH_YIELD,
      TileType::Saplings => &SAPLINGS_TILE_WITH_YIELD,
      TileType::Moss => &MOSS_TILE_WITH_YIELD,
      TileType::Clover => &CLOVER_TILE_WITH_YIELD,
      TileType::BerryBush => &BERRYBUSH_TILE_WITH_YIELD,
      TileType::Ferns => &FERNS_TILE_WITH_YIELD,
      TileType::Birch => &BIRCH_TILE_WITH_YIELD,
      TileType::Pine => &PINE_TILE_WITH_YIELD,
      TileType::Oak => &OAK_TILE_WITH_YIELD,
      TileType::GreenPuddle => &GREENPUDDLE_TILE_WITH_YIELD,
      TileType::CoarseDirt => &COARSEDIRT_TILE_WITH_YIELD,
      TileType::LeafLitter => &LEAFLITTER_TILE_WITH_YIELD,
    }
  }

  pub fn graphic_without_yield(&self) -> &'static GraphicAsset {
    match self {
      TileType::Ash => &ASH_TILE_WITHOUT_YIELD,
      TileType::CharredFallenLog => &CHARREDFALLENLOG_TILE_WITHOUT_YIELD,
      TileType::CharredTreeTrunk => &CHARREDTREETRUNK_TILE_WITHOUT_YIELD,
      TileType::CharredGrass => &CHARREDGRASS_TILE_WITHOUT_YIELD,
      TileType::Puddle => &PUDDLE_TILE_WITHOUT_YIELD,
      TileType::DryDirt => &DRYDIRT_TILE_WITHOUT_YIELD,
      TileType::Dirt => &DIRT_TILE_WITHOUT_YIELD,
      TileType::Grass => &GRASS_TILE_WITHOUT_YIELD,
      TileType::Shrub => &SHRUB_TILE_WITHOUT_YIELD,
      TileType::Flowers => &FLOWERS_TILE_WITHOUT_YIELD,
      TileType::Saplings => &SAPLINGS_TILE_WITHOUT_YIELD,
      TileType::Moss => &MOSS_TILE_WITHOUT_YIELD,
      TileType::Clover => &CLOVER_TILE_WITHOUT_YIELD,
      TileType::BerryBush => &BERRYBUSH_TILE_WITHOUT_YIELD,
      TileType::Ferns => &FERNS_TILE_WITHOUT_YIELD,
      TileType::Birch => &BIRCH_TILE_WITHOUT_YIELD,
      TileType::Pine => &PINE_TILE_WITHOUT_YIELD,
      TileType::Oak => &OAK_TILE_WITHOUT_YIELD,
      TileType::GreenPuddle => &GREENPUDDLE_TILE_WITHOUT_YIELD,
      TileType::CoarseDirt => &COARSEDIRT_TILE_WITHOUT_YIELD,
      TileType::LeafLitter => &LEAFLITTER_TILE_WITHOUT_YIELD,
    }
  }
}
