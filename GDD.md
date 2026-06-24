# GDD

This document serves as the absolute technical manual and structural blueprint for **Fungal Economics: Spore War**, a hybrid strategy game about forest succession and fungal economics designed to be played in under **20-minutes**.

## 1. High-Level Concept & Learning Architecture

- **The Topic:** Fungal Networks & Succession
- **The Learning Goal:** Players discover how competing fungal networks must dynamically adapt their subterranean growth and resource investments throughout the five distinct stages of forest succession to secure limited resources.
- **Hybrid Framework:** The **Physical Board** tracks spatial geometry, physical path blockades, and player turf layouts. The **Individual Web App Dashboard** (running on each player's personal device) handles complex metabolic calculations, resource pools, automated trade validations, and the global game clock. All players must hold the `START` button for 10s at the same time, there is no multiplayer.
- **Target Audience:** Classroom environments split into parallel competitive groups of 4-6 players per board.
- **Core Biological Principle:** Fungi are heterotrophs. They do not generate their own food; they must either decompose dead matter (saprotrophy) or trade soil resources with living plants for photosynthetic sugars (mycorrhizal symbiosis). Expansion requires Carbon (C) for cellular structure and Water (H₂O) to generate the internal turgor pressure required to physically push through soil grains.

## 2. Physical Components & Map Architecture

### The Multi-Layered 61-Hex Board

The physical map consists of a multi-layered frame (constructed from durable HDF/MDF wood plates or thick cardboard) housing **5 stacked full-board cardboard layers**, one for each phase of succession. The grid layout features a central hex tile surrounded by 4 complete concentric rings, totaling exactly **61 hex spaces** (1 + 6 + 12 + 18 + 24 = 61) per layer.

- **The Layer-Shift Mechanism:** Each of the 5 board layers features a numbered indexing tab (1 through 5) extending from its outer edge. When a phase transition occurs, players simply pull out the new sheet and put it at the top of the stack. This completely updates the visible landscape across all 61 spaces simultaneously without disturbing the tokens currently placed on the board.
- **Tile Design Standard:** Each hex space directly features its artwork and has a trade/yield formulation (e.g. `3N -3P → +2N`) in the center. To allow the web app to function without tracking exact grid coordinates, **all tiles of the exact same type follow an identical path of succession across the layered sheets**. This succession is shown as a chart in the manual.

### The 3D Stacking Tokens

Each player is assigned a distinct, highly visible color (e.g., Neon Blue, Orange, Pink, Green). The tokens are 3D-printed and here's a prototype, missing the stacking functionality:

![temp_1781854121877.jpg](GDD/temp_1781854121877.jpg)

- The pieces are shaped like rings with a wide open center hole. When stacked up, players can look directly down through the center of the stack to read the tile type and its trade/yield printed in the center of the board layer beneath.
- The main ring aside from the the 6 connections waves vertically and has little organic tentacles going upwards, that plug into small holes in the bottom of other pieces. This allows the rings to snap cleanly onto each other, ensuring perfect physical stability when players jostle the table.

## 3. Core Mechanical Systems & Game Rules

### Global Resource Limits & Starting State

- Players chose a corner to put their first token in for free. All 6 corners start as Puddle to give players equal chances and so the app already knows the first tile.
- **Resource Cap:** For user interface cleanlines, a player's inventory can hold a maximum of **9999** units of any individual resource.
- **Starting Capital:** Each player starts Phase 1 logged into their app with:
- 200 Carbon (C)
- 50 Nitrogen (N)
- 50 Phosphorus (P)
- 50 Water (H₂O)

### The Rules of Movement & Expansion

- Spreading your network to any adjacent hex costs a flat rate of **100 Carbon**.
- To expand, you must also spend Water to simulate turgor pressure. The drier or denser the target terrain, the higher the water cost:
- Damp Terrain: 10 Water
- Standard Terrain: 25 Water
- Dense Terrain: 50 Water

### Outstacking

- Players can expand horizontally or vertically onto their own tiles allowing them to outstack their opponent.
- To stack above the opponent's token the player first needs to build a stack on an adjacent tile such that they can spread horizontally onto the opponents tokens.
- Stacks can be a maximum of 4 tokens high.
- Only the player whose token is at the absolute top of the stack is considered active.
- Because tokens need to be directly connected, you can't expand onto a lower height

Because the web app operates without a map coordinate tracking system, the players handle turf takeovers through a manual reporting feature:

1. Player A presses the button to pay the expansion fee and start the yield from that tile.
2. Player A physically drops their token on top of Player B's token on the board.
3. Player A or B immediately press the **[OUTSTACKED]** button on Player B's app. The app displays a menu of their currently active tile types. When selected the app stops the yield from that specific tile.

## 4. The 5-Phase Succession & Economic Balance Matrix

The game is a fully deterministic simulation. The match moves from a wide-open, mineral-rich post-fire wasteland into a hyper-dense, highly competitive climax forest canopy.

### Complete Deterministic Succession Master Table

| Base Tile (Count) | → Phase 2 | → Phase 3 | → Phase 4 | Phase 5 |
| --- | --- | --- | --- | --- |
| Ash (15) | II: Dirt | III: Moss | IV: Pine | V: Oak |
| Charred Fallen Log (12) | II: Grass | III: Clover | IV: Pine | V: Oak |
| Charred Tree Trunk (10) | II: Shrub | III: Berry Bush | IV: Oak | - |
| Charred Grass (10) | II: Flowers | III: Ferns | IV: Oak | - |
| Puddle (8) | - | - | IV: Green Puddle | V: Leaf Litter |
| Dry Dirt (6) | II: Saplings | III: Birch | IV: Coarse Dirt | V: Leaf Litter |

### Phase Economic Sheets

For now: 1 tick = 5s

### Phase 1: The Saprotrophic Ash Bed

*Context: Immediate aftermath of a wildfire. No living plants exist. Fungi act as decomposers, burning resources to break down dead wood, or mining ash for minerals.*

- **Puddle (Start Tile):** Cost: 100 Carbon + 10 Water | Yield: +15 Water per tick.
- **Ash (15):** Cost: 100 Carbon + 25 Water | Yield: +20 Carbon, +3 Nitrogen, +5 Phosphorus per tick.
- **Charred Fallen Log (12):** Cost: 100 Carbon + 10 Water | Decomposition Upkeep: Consumes 2 Nitrogen, 1 Water per tick | Yield: +20 Carbon per tick.
- **Charred Tree Trunk (10):** Cost: 100 Carbon + 50 Water | Decomposition Upkeep: Consumes 4 Phosphorus, 2 Water per tick | Yield: +25 Carbon per tick.
- **Charred Grass (10):** Cost: 100 Carbon + 25 Water | Yield: +12 Carbon, +4 Nitrogen, +10 Phosphorus per tick.
- **Dry Dirt (6):** Cost: 100 Carbon + 25 Water | Yield: +10 Carbon, +6 Nitrogen, +4 Phosphorus, +4 Water per tick.

### Phase 2: The Undergrowth Spark

*Context: Pioneer plants sprout. Decomposition yields end; the Mycorrhizal Symbiotic Trade Engine activates. Fungi must now feed plants minerals/water to receive Carbon sugars in return.*

- **Puddle (8):** Cost: 100 Carbon + 10 Water | Yield: +15 Water per tick.
- **Dirt (15):** Cost: 100 Carbon + 25 Water | Yield: +20 Nitrogen, +20 Phosphorus, +2 Water per tick.
- **Grass (12):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 3 Nitrogen, 3 Water per tick | Yield: +20 Carbon per tick.
- **Shrub (10):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 4 Phosphorus, 3 Water per tick | Yield: +30 Carbon per tick.
- **Flowers (10):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 2 Nitrogen, 2 Phosphorus per tick | Yield: +30 Carbon, +4 Water per tick.
- **Saplings (6):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 4 Nitrogen, 2 Phosphorus, 4 Water per tick | Yield: +40 Carbon per tick.

### Phase 3: The Early Canopy Thicket

*Context: Vegetation matures and crowds the surface. Ground-covering moss, fixers, and ferns act as vital mineral and nutrient-cycling powerhouses before the shade of the canopy closes in.*

- **Puddle (8):** Cost: 100 Carbon + 10 Water | Yield: +15 Water per tick.
- **Moss (15):** Cost: 100 Carbon + 25 Water | Yield: +12 Carbon, +15 Nitrogen, +10 Phosphorus, +8 Water per tick.
- **Clover (12):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 2 Phosphorus, 2 Water per tick | Yield: +20 Carbon, +12 Nitrogen per tick.
- **Ferns (10):** Cost: 100 Carbon + 15 Water | Symbiotic Trade: Consumes 2 Water per tick | Yield: +15 Carbon, +8 Nitrogen, +12 Phosphorus per tick.
- **Berry Bush (10):** Cost: 100 Carbon + 25 Water | Symbiotic Trade: Consumes 6 Phosphorus, 2 Nitrogen, 2 Water per tick | Yield: +35 Carbon per tick.
- **Birch (6):** Cost: 100 Carbon + 35 Water | Symbiotic Trade: Consumes 10 Nitrogen, 8 Phosphorus, 6 Water per tick | Yield: +45 Carbon per tick.

### Phase 4: The Closed Canopy Crunch

*Context: The canopy closes, shading out undergrowth. The landscape transitions into a heavy timber environment dominated by 27 Pines and 20 early Oaks.*

- **Pine (27):** Cost: 100 Carbon + 35 Water | Symbiotic Trade: Consumes 15 Nitrogen, 10 Phosphorus, 8 Water per tick | Yield: +50 Carbon per tick.
- **Oak (20):** Cost: 100 Carbon + 50 Water | Symbiotic Trade: Consumes 20 Nitrogen, 20 Phosphorus, 15 Water per tick | Yield: +100 Carbon per tick.
- **Green Puddle (8):** Cost: 100 Carbon + 10 Water | Yield: +20 Carbon, +30 Nitrogen, +20 Phosphorus, +20 Water per tick.
- **Coarse Dirt (6):** Cost: 100 Carbon + 25 Water | Yield: +2 Carbon, +15 Nitrogen, +20 Phosphorus, +2 Water per tick.

### Phase 5: Climax Forest Dominance

*Context: Old-growth climax forest. The map collapses into a strict binary layout: 47 massive Oaks and 14 floor-covering Leaf Litters.*

- **Oak (47):** Cost: 100 Carbon + 50 Water | Symbiotic Trade: Consumes 20 Nitrogen, 20 Phosphorus, 15 Water per tick | Yield: +100 Carbon per tick.
- **Leaf Litter (14):** Cost: 100 Carbon + 25 Water | Yield: +5 Carbon, +20 Nitrogen, +20 Phosphorus, +8 Water per tick.

## 5. Software Application Logic & Interface Rules

### Interface Rules

- The dashboard screen displays current ledger balances (Carbon, Nitrogen, Phosphorus, Water) and purchased **Spore Points**. It is actually shown the floor of the balances as they are floats and increase continually.
- The expansion section shows a grid of buttons listing *only the tile types available in the current phase*. The button shows the entire tile.
- Each button features its explicit cost directly beneath it (`for -100C -25H2O`) and shows the trade/yield overview on it (i.e. `3N -3P → +2N` or `+2N`), the same as is on the tiles.
- If a player's wallet falls below the required threshold for an action, the missing resource count turns vibrant **red** in the cost, and the button is immediately **greyed out** and unclickable.
- The player should first input into the app and then physically place their token.

### Automated Phase Clock & Sync Loops

- The web app runs a master countdown clock divided into five 2-minute blocks (5 x 2 = 10 minutes total), 10 minutes left for rule explaining and changing the board.
- When a 2-minute block expires, the app pauses the game clock, makes a sound and switches to transition mode.
- It instructs the players to adjust the physical board frame, reordering the cardboard layers so that the indexing tab corresponding to the upcoming phase is resting at the top.
- Then players hold the `READY` button on their phones at the same time for 10s (there is no multiplayer). Once all players have confirmed, the next phase activates, and the app mutates all existing claimed ledger assets automatically to match the new succession matrix.

### The Symbiotic Trade Breakdown Rule

On every single ledger calculation tick, the app runs an unyielding conditional check for any active nodes requiring resource inputs (e.g., Log/Trunk decomposition upkeeps or Plant symbiotic demands):

If (Current Inventory Pool) >= (Required Node Upkeep Cost) then Deduct Upkeep, Grant Full Carbon Reward.

If (Current Inventory Pool) < (Required Node Upkeep Cost) then Convert & Drain Remaining Fraction to 0.

If a player falls short on even a single required mineral or water point, the full nutrients can’t be dispensed. Nodes are traded with in the order they were added to the app. In the app the missing resource becomes a glowing red 0.

## 6. Endgame Condition & Victory Evaluation

- **The Ultimate Goal:** Fungi expand to maximize genetic dispersal via spores. Spores are purchased on the app screen via the `SPORE` button at a steep premium:

1 Spore Point = 500 Carbon + 50 Nitrogen + 50 Phosphorus + 50 Water

- **Partial Sporing:** Because in the end game ressources might go into the red for trading situations the spore button should be able to be hold to continually prioritize investing into sporing.
- **Winning the Game:** When Phase 5's timer hits zero, the app completely locks down. The player who has accumulated the highest number of **Spore Points** on their app ledger wins the match.
- **The Tiebreaker:** Players decide on their own.

## 7. Crucial Guidelines for the Manual Writer

To maximize the game's academic evaluation metrics, the rulebook writer must explicitly emphasize these structural bottlenecks within the text:

1. **The Phase 5 Trap:** The manual must warn players that **Oaks are a double-edged sword**. They produce a massive +400 Carbon, but their upkeep requirement (40/40/40) is enormous. Because there are 47 Oaks but only 14 Leaf Litter nutrient sources in Phase 5, players must use foresight in Phase 3 and 4 to hoard extra Water, Nitrogen, and Phosphorus. Entering Phase 5 with empty wallets and multiple Oak claims will trigger a catastrophic systemic breakdown, freezing their network completely.
2. **The Stacking War:** The manual needs to frame vertical stacking as a high-friction defensive action. Stacking forces players into close-quarters territorial combat over the few remaining Leaf Litter tiles, driving the fast-paced, cutthroat table dynamic required for a 20-minute game window.
3. The manual needs to show a nice succession chart.