# Architectural Blueprint & Planning Manual

## Fungal Economics: Spore War Dashboard (Ply Engine Edition)

This document establishes the comprehensive technical architecture, state model, and UI/UX layouts for the **Fungal Economics: Spore War** mobile companion dashboard. Built on top of the `ply-engine` framework, this design optimizes for zero-latency frame rendering, deterministic mathematical consistency, and bulletproof user interactions within a rapid 20-minute gameplay window.

---

## 1. Core State Architecture & Memory Layout

Because the application must run with a decoupled grid coordinate space while accurately maintaining metabolic calculations, the state machine utilizes a centralized ledger model.

### Data Models (`src/models.rs`)

```rust
// This should get impls for Add other, Sub other, AddAssign, SubAssign, Mul by f32, Div by f32... so you can do math on Resources directly, which makes stuff easier, like the math needed below.
pub struct Resources {
    pub carbon: f32,
    pub nitrogen: f32,
    pub phosphorus: f32,
    pub water: f32,
}
// this impl should have a function minimum_fraction_fulfilled(&self, cost: &Resources) -> (f32, IsResourceMissing), more on that later.

pub enum BaseTileType {
    Ash,
    CharredFallenLog,
    CharredTreeTrunk,
    CharredGrass,
    Puddle,
    DryDirt,
}

// There should be a static lookup table to convert BaseTileType+Phase -> TileType.
// The impl for BaseTileType should have a method to return the current TileType given the current phase index.

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

// There should also be a static lookup table for the cost of water expansion for each TileType, one for the trade each offers and one for the image GraphicAsset each has.
// The impl for TileType should have methods to return the water expansion cost and the trade for that tile type.

pub struct Trade {
    pub consumes_per_tick: Option<Resources>,
    pub yields_per_tick: Resources,
}

pub struct IsResourceMissing {
    pub carbon: bool,
    pub nitrogen: bool,
    pub phosphorus: bool,
    pub water: bool,
}
// This should impl BitOr and BitOrAssign

pub struct GameState {
    pub is_paused: bool,
    pub current_phase: u8,
    pub phase_timer: f32,
    pub resource_pool: Resources,
    pub is_resource_missing: IsResourceMissing,
    pub is_overstacked_menu_opened: bool,
    pub nodes: Vec<BaseTileType>,
    pub spore_points: u32,
}

pub static SPORE_POINT_COSTS: Resources = Resources {
    carbon: 400.0,
    nitrogen: 100.0,
    phosphorus: 100.0,
    water: 100.0,
};

pub static TICK_LENGTH: f32 = 5.0; // seconds

pub static PHASE_LENGTH: f32 = 120.0; // seconds

pub enum GameMode { // This is what saves the data!
    StartSync { hold_accumulation: f32 },
    Playing { state: GameState },
    TransitionSync { state: GameState, hold_accumulator: f32 },
    GameOver { state: GameState },
}
```

## 2. The Continuous Metabolic Engine (Delta-Time Processing)

Resources accumulate and deplete smoothly over time rather than via abrupt step-functions. This prevents visual jarring and creates a real-time metabolic feedback system.

### Fractional Accounting Formulas

Calculations scale uniformly across standard rendering environments:

* **Tick Length Constant:** 5.0 seconds.
* **Frame Progression Scaling Factor:** $\Delta t / TICK_LENGTH$

### Sequential Upkeep Processing & Fractional Drainage Execution

During execution, the calculation loop updates state via the following pipeline:

1. For each node in the `active_nodes` array, retrieve its `Trade` struct.
2. If `consumes_per_tick` is Some:
   1. Multiply by delta time to get the actual `consumptions` for this frame.
   2. This is called `resource_pool.minimum_fraction_fulfilled(consumptions)`.
       1. For each of the ressources calculate min(owned/consumtion, 1.0) to determine the fraction of consumption that can be satisfied.
       2. Take the minimum fraction across all resources.
   c. You also get `IsResourceMissing` flags, you need to bit-or-assign those to the `GameState.is_resource_missing`.
   d. Subtract that fraction of the `consumptions` from the `resource_pool`.
3. Multiply the `yields_per_tick` by delta time to get the actual `yields` for this frame.
4. Add that fraction (or 1.0, if `consumes_per_tick` is None) of the `yields` to the `resource_pool`.


## 3. Deterministic Succession & Phase Transition Logic

Transitions between the 5 phases of forest succession are handled via structural mutations applied across the array of active nodes.

### Transition Step Pipeline

1. **Clock Depletion:** The 2-minute phase countdown reaches 0.
2. **Intermission State:** The runtime engine stops the game clock and activates an audio alert cue.
3. **Physical Synchronization:** The UI switches to only displays a prompt directing players to lift and re-index the physical board layer.
4. **Joint Confirmation Verification:** The interface renders a sync button. All group members must place and hold their fingers on this input for 10 consecutive seconds.
5. **State Mutation Phase:** Upon successful verification, the engine continues to the next phase.

## 4. UI/UX Component & Layout Blueprint

The user interface follows a clean mobile-first layout strategy designed to minimize distraction and reduce input errors during intensive gameplay.

```
+-------------------------------------------------------------+
| [PHASE 3]                                        01:42      |
| 1,420 C, 84 N, 12 P, 312 H2O                    2 SP        |
+-------------------------------------------------------------+
| +-------------------------+   +---------------------------+ |
| |      IMAGE              |   |        IMAGE              | |
| |    +8C / tick           |   |    -3N, -3P -> +16C       | |
| |    for -100C -25H2O     |   |    for -100C -25H2O       | |
| +-------------------------+   +---------------------------+ |
| |        IMAGE            |   |           IMAGE           | |
| |    -4N,-2P,-2H2O->+22C  |   |    -5P -> +14C            | |
| |    for -100C -25H2O     |   |    for -100C -15H2O       | |
| +-------------------------+   +---------------------------+ |
| |          IMAGE          |   |         IMAGE             | |
| |    +30H2O / tick        |   |    -6N, -6H2O -> +35C     | |
| |    for -100C -10H2O     |   |    [OUT OF WATER DEFICIT] | |
| +-------------------------+   +---------------------------+ |
+-------------------------------------------------------------+
| [Outstack]                    [Spore]                       |
+-------------------------------------------------------------+
```

### Layout Properties

* **Resource Display Top Bar:** Renders the floor integer value of each asset pool using a standard typographical block. This ensures clear scannability and prevents fractional flickering.
* **The Grid Interface Engine:**
  * **Narrow Viewports (Mobile Portrait Mode):** Renders as a 2-column wide by 3-row high element grid block to protect thumb accessibility.
  * **Wide Viewports (Tablets / Mobile Landscape Mode):** Flexes automatically into a 3-column wide by 2-row high grid pattern.
  * Other sizes should make sure one of these orientations fits by adding spacing around the grid. The grid should never be stretched to fill the entire width of the screen, and should always be centered horizontally.
  * If it's only 4 types, it should be 2x2 in any case.
  * If it's only 2 types, it should be 1x2 in any case.


* **Affordance Visual States:** Every node-buying action block evaluates its internal activation conditions against the current asset wallet. If a pool falls below the activation requirement:
1. The button container drops its operational contrast and switches to an unclickable grayed-out layout state.
2. The deficient asset line updates its text configuration string, rendering the specific missing requirement using a vibrant red highlight style.

* **Missing Resource Alerts:** If the player is missing a ressource to keep up with consumption (is_resource_missing), that ressource should be highlighted in red in the top bar. 

### Outstack Reporting Flow

Because the digital app acts as a companion without real-time board tracking features, network losses are handled via a manual resolution system:

1. When an opponent overrides a player's physical token, that player taps the main `[OUTSTACKED]` button.
2. The application opens an overlay showing a grid such as the main one, but with the tiles that are not in the player's possession grayed out and unclickable.
3. Selecting a tile highlights it.
4. Pressing the `[CONFIRM]` button triggers the removal process. This locates the oldest matching active instance inside the array list and removes it, successfully halting its future yield and consumption loops. The overlay closes, returning the player to the main dashboard. 
5. There is also a `[CANCEL]` button to exit the overlay without making any changes.

## 5. Strategic Bottlenecks & Protective Systems

To safeguard game balance and create deep strategic tension over the 20-minute match length, the system implements specific defensive guardrails.