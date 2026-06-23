use ply_engine::prelude::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, BitOr, BitOrAssign};

static TEST_IMAGE: GraphicAsset = GraphicAsset::Bytes { file_name: "test.png", data: include_bytes!("../assets/images/test.png") };

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
  RegularDirt,
  PioneerGrass,
  LowShrub,
  Flowers,
  Saplings,
  SproutingGrass,
  WoodyShrub,
  BerryBush,
  Ferns,
  YoungPine,
  FastPine,
  AncientOak,
  GreenPuddle,
  CoarseDirt,
  LeafLitter,
}

pub struct Trade {
  pub consumes_per_tick: Option<Resources>,
  pub yields_per_tick: Resources,
}

#[derive(Clone, Debug)]
pub struct GameState {
  pub is_paused: bool,
  pub current_phase: u8, // 1 to 5
  pub phase_timer: f32,
  pub resource_pool: Resources,
  pub is_resource_missing: IsResourceMissing,
  pub is_overstacked_menu_opened: bool,
  pub active_nodes: Vec<BaseTileType>,
  pub spore_points: u32,
}

#[derive(Clone, Debug)]
pub enum GameMode {
  StartSync { hold_accumulation: f32 },
  Playing { state: GameState },
  TransitionSync { state: GameState, hold_accumulator: f32 },
  GameOver { state: GameState },
}

pub const SPORE_POINT_COSTS: Resources = Resources {
  carbon: 400.0,
  nitrogen: 100.0,
  phosphorus: 100.0,
  water: 100.0,
};

pub const TICK_LENGTH: f32 = 5.0;
pub const PHASE_LENGTH: f32 = 120.0;
pub const START_CARBON: f32 = 200.0;
pub const START_NITROGEN: f32 = 50.0;
pub const START_PHOSPHORUS: f32 = 50.0;
pub const START_WATER: f32 = 50.0;
pub const SYNC_HOLD_TIME: f32 = 10.0;

impl BaseTileType {
  pub fn get_current_tile_type(&self, phase: u8) -> TileType {
    match self {
      BaseTileType::Ash => match phase {
        1 => TileType::Ash,
        2 => TileType::RegularDirt,
        3 => TileType::SproutingGrass,
        4 => TileType::FastPine,
        _ => TileType::AncientOak,
      },
      BaseTileType::CharredFallenLog => match phase {
        1 => TileType::CharredFallenLog,
        2 => TileType::PioneerGrass,
        3 => TileType::WoodyShrub,
        4 => TileType::FastPine,
        _ => TileType::AncientOak,
      },
      BaseTileType::CharredTreeTrunk => match phase {
        1 => TileType::CharredTreeTrunk,
        2 => TileType::LowShrub,
        3 => TileType::BerryBush,
        _ => TileType::AncientOak,
      },
      BaseTileType::CharredGrass => match phase {
        1 => TileType::CharredGrass,
        2 => TileType::Flowers,
        3 => TileType::Ferns,
        _ => TileType::AncientOak,
      },
      BaseTileType::Puddle => match phase {
        1 | 2 | 3 => TileType::Puddle,
        4 => TileType::GreenPuddle,
        _ => TileType::LeafLitter,
      },
      BaseTileType::DryDirt => match phase {
        1 => TileType::DryDirt,
        2 => TileType::Saplings,
        3 => TileType::YoungPine,
        4 => TileType::CoarseDirt,
        _ => TileType::LeafLitter,
      },
    }
  }
}

impl TileType {
  pub fn water_cost(&self) -> f32 {
    match self {
      TileType::Puddle | TileType::CharredFallenLog | TileType::Ferns | TileType::LeafLitter | TileType::GreenPuddle => 10.0,
      TileType::Ash | TileType::CharredGrass | TileType::DryDirt | TileType::RegularDirt | TileType::PioneerGrass | TileType::LowShrub | TileType::Flowers | TileType::Saplings | TileType::SproutingGrass | TileType::WoodyShrub | TileType::BerryBush | TileType::CoarseDirt => 25.0,
      TileType::CharredTreeTrunk | TileType::YoungPine | TileType::FastPine | TileType::AncientOak => 50.0,
    }
  }

  pub fn expansion_carbon_cost(&self) -> f32 {
    100.0
  }

  pub fn get_trade(&self) -> Trade {
    match self {
      TileType::Ash => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 3.0, 3.0, 0.0),
      },
      TileType::CharredFallenLog => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 2.0, 0.0, 0.0)),
        yields_per_tick: Resources::new(15.0, 0.0, 0.0, 0.0),
      },
      TileType::CharredTreeTrunk => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 4.0, 0.0)),
        yields_per_tick: Resources::new(25.0, 0.0, 0.0, 0.0),
      },
      TileType::CharredGrass => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(3.0, 1.0, 1.0, 0.0),
      },
      TileType::Puddle => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 0.0, 0.0, 30.0),
      },
      TileType::DryDirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 1.0, 1.0, 1.0),
      },
      TileType::RegularDirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 2.0, 2.0, 2.0),
      },
      TileType::PioneerGrass => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 0.0, 2.0)),
        yields_per_tick: Resources::new(6.0, 0.0, 0.0, 4.0),
      },
      TileType::LowShrub => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 2.0, 2.0, 0.0)),
        yields_per_tick: Resources::new(12.0, 0.0, 0.0, 2.0),
      },
      TileType::Flowers => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 4.0, 0.0)),
        yields_per_tick: Resources::new(10.0, 0.0, 0.0, 7.0),
      },
      TileType::Saplings => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 3.0, 0.0, 3.0)),
        yields_per_tick: Resources::new(15.0, 0.0, 0.0, 0.0),
      },
      TileType::SproutingGrass => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 0.0, 3.0)),
        yields_per_tick: Resources::new(8.0, 0.0, 0.0, 0.0),
      },
      TileType::WoodyShrub => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 3.0, 3.0, 0.0)),
        yields_per_tick: Resources::new(16.0, 0.0, 0.0, 0.0),
      },
      TileType::BerryBush => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 4.0, 2.0, 2.0)),
        yields_per_tick: Resources::new(22.0, 0.0, 0.0, 0.0),
      },
      TileType::Ferns => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 0.0, 5.0, 0.0)),
        yields_per_tick: Resources::new(14.0, 0.0, 0.0, 0.0),
      },
      TileType::YoungPine => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 6.0, 0.0, 6.0)),
        yields_per_tick: Resources::new(35.0, 0.0, 0.0, 0.0),
      },
      TileType::FastPine => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 10.0, 0.0, 10.0)),
        yields_per_tick: Resources::new(60.0, 0.0, 0.0, 0.0),
      },
      TileType::AncientOak => Trade {
        consumes_per_tick: Some(Resources::new(0.0, 40.0, 40.0, 40.0)),
        yields_per_tick: Resources::new(400.0, 0.0, 0.0, 0.0),
      },
      TileType::GreenPuddle => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 2.0, 0.0, 30.0),
      },
      TileType::CoarseDirt => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 3.0, 3.0, 0.0),
      },
      TileType::LeafLitter => Trade {
        consumes_per_tick: None,
        yields_per_tick: Resources::new(0.0, 15.0, 15.0, 15.0),
      },
    }
  }

  pub fn label(&self) -> &'static str {
    match self {
      TileType::Ash => "Ash",
      TileType::CharredFallenLog => "Charred Log",
      TileType::CharredTreeTrunk => "Charred Trunk",
      TileType::CharredGrass => "Charred Grass",
      TileType::Puddle => "Puddle",
      TileType::DryDirt => "Dry Dirt",
      TileType::RegularDirt => "Regular Dirt",
      TileType::PioneerGrass => "Pioneer Grass",
      TileType::LowShrub => "Low Shrub",
      TileType::Flowers => "Flowers",
      TileType::Saplings => "Saplings",
      TileType::SproutingGrass => "Sprouting Grass",
      TileType::WoodyShrub => "Woody Shrub",
      TileType::BerryBush => "Berry Bush",
      TileType::Ferns => "Ferns",
      TileType::YoungPine => "Young Pine",
      TileType::FastPine => "Fast Pine",
      TileType::AncientOak => "Ancient Oak",
      TileType::GreenPuddle => "Green Puddle",
      TileType::CoarseDirt => "Coarse Dirt",
      TileType::LeafLitter => "Leaf Litter",
    }
  }

  pub fn icon(&self) -> &'static GraphicAsset {
    match self {
      _ => &TEST_IMAGE, // TODO: Replace with actual icons for each tile type
    }
  }
}
