use crate::models::*;
use ply_engine::prelude::*;

pub const COLOR_CARBON: u32 = 0xBDBDBD; // Gray-ish
pub const COLOR_NITROGEN: u32 = 0x4CAF50; // Green
pub const COLOR_PHOSPHORUS: u32 = 0xFFC107; // Amber
pub const COLOR_WATER: u32 = 0x2196F3; // Blue
pub const COLOR_RED: u32 = 0xF44336;
pub const COLOR_BG: u32 = 0x121212;
pub const COLOR_CARD_BG: u32 = 0x1E1E1E;

pub struct Assets {
  pub test_image: &'static GraphicAsset,
  pub next_sound: Sound,
  pub pause_sound: Sound,
}

pub fn render_ui(ui: &mut Ui, mode: &mut GameMode, assets: &Assets) {
  ui.element().width(grow!()).height(grow!())
    .background_color(COLOR_BG)
    .children(|ui| {
      match mode {
        GameMode::StartSync { hold_accumulation } => {
          render_sync_screen(ui, "HOLD TO START SYNC", *hold_accumulation);
        }
        GameMode::Playing { state } => {
          render_dashboard(ui, state, assets);
          if state.is_overstacked_menu_opened {
            render_outstack_overlay(ui, state, assets);
          }
        }
        GameMode::TransitionSync { state: _, hold_accumulator } => {
          render_sync_screen(ui, "PHASE TRANSITION: HOLD TO SYNC", *hold_accumulator);
        }
        GameMode::GameOver { state } => {
          render_game_over(ui, state);
        }
      }
    });
}

fn render_sync_screen(ui: &mut Ui, message: &str, progress: f32) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(20))
    .children(|ui| {
      ui.text(message, |t| t.font_size(24).color(0xFFFFFF));
      
      let p = (progress / SYNC_HOLD_TIME).clamp(0.0, 1.0);
      ui.element().width(fixed!(300.0)).height(fixed!(20.0))
        .background_color(0x333333)
        .children(|ui| {
          ui.element().width(fixed!(300.0 * p)).height(grow!())
            .background_color(COLOR_WATER).empty();
        });

      ui.text("All players must hold simultaneously", |t| t.font_size(14).color(0xAAAAAA));
    });
}

fn render_dashboard(ui: &mut Ui, state: &mut GameState, assets: &Assets) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.direction(TopToBottom))
    .children(|ui| {
      render_top_bar(ui, state);

      ui.element().width(grow!()).height(grow!(weight: 1.0))
        .overflow(|o| o.scroll_y())
        .layout(|l| l.align(CenterX, CenterY).padding(20))
        .children(|ui| {
          render_grid(ui, state, assets);
        });

      render_bottom_bar(ui, state);
    });
}

fn render_top_bar(ui: &mut Ui, state: &GameState) {
  ui.element().width(grow!()).height(fit!())
    .background_color(0x000000)
    .layout(|l| l.direction(TopToBottom).padding(10).gap(5))
    .children(|ui| {
      // Title and Timer
      ui.element().width(grow!()).height(fit!())
        .children(|ui| {
          ui.text(&format!("PHASE {}", state.current_phase), |t| t.font_size(18).color(0xFFFFFF));
          ui.element().width(grow!()).empty();
          let mins = (state.phase_timer as i32) / 60;
          let secs = (state.phase_timer as i32) % 60;
          ui.text(&format!("{:02}:{:02}", mins, secs), |t| t.font_size(18).color(0xFFFFFF));
        });

      // Resources
      ui.element().width(grow!()).height(fit!())
        .layout(|l| l.gap(15))
        .children(|ui| {
          resource_label(ui, "C", state.resource_pool.carbon, state.is_resource_missing.carbon, COLOR_CARBON);
          resource_label(ui, "N", state.resource_pool.nitrogen, state.is_resource_missing.nitrogen, COLOR_NITROGEN);
          resource_label(ui, "P", state.resource_pool.phosphorus, state.is_resource_missing.phosphorus, COLOR_PHOSPHORUS);
          resource_label(ui, "H", state.resource_pool.water, state.is_resource_missing.water, COLOR_WATER);
          
          ui.element().width(grow!()).empty();
          ui.text(&format!("{} SP", state.spore_points), |t| t.font_size(18).color(0xFFFFFF));
        });
    });
}

fn resource_label(ui: &mut Ui, label: &str, value: f32, is_missing: bool, color: u32) {
  let display_color = if is_missing { COLOR_RED } else { color };
  ui.element().width(fit!()).height(fit!())
    .layout(|l| l.gap(2))
    .children(|ui| {
      ui.text(&format!("{}", value.floor() as i32), |t| t.font_size(16).color(display_color));
      ui.text(&format!("{}", label), |t| t.font_size(14).color(0xFFFFFF));
    });
}

fn render_grid(ui: &mut Ui, state: &mut GameState, assets: &Assets) {
  let available_bases = match state.current_phase {
    1 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
    2 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
    3 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::CharredTreeTrunk, BaseTileType::CharredGrass, BaseTileType::Puddle, BaseTileType::DryDirt],
    4 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::Puddle, BaseTileType::DryDirt],
    5 => vec![BaseTileType::Ash, BaseTileType::CharredFallenLog, BaseTileType::Puddle, BaseTileType::DryDirt],
    _ => vec![],
  };

  let cols = if screen_width() > screen_height() { 3 } else { 2 };

  ui.element().width(fit!()).height(fit!())
    .layout(|l| l.direction(TopToBottom).gap(15).align(CenterX, CenterY))
    .children(|ui| {
      for row in available_bases.chunks(cols as usize) {
        ui.element().width(fit!()).height(fit!())
          .layout(|l| l.gap(15))
          .children(|ui| {
            for base in row {
              render_tile_button(ui, state, *base, assets);
            }
          });
      }
    });
}

fn render_tile_button(ui: &mut Ui, state: &mut GameState, base: BaseTileType, assets: &Assets) {
  let tile = base.get_current_tile_type(state.current_phase);
  let cost_c = tile.expansion_carbon_cost();
  let cost_w = tile.water_cost();
  let can_afford = state.resource_pool.carbon >= cost_c && state.resource_pool.water >= cost_w;
  let trade = tile.get_trade();
  let id = Id::new_index("tile_btn", base as u32);

  ui.element().width(fixed!(140.0)).height(fixed!(180.0))
    .id(id.clone())
    .background_color(COLOR_CARD_BG)
    .corner_radius(8.0)
    .border(|b| b.all(if can_afford { 1 } else { 0 }).color(0x333333))
    .layout(|l| l.direction(TopToBottom).padding(8).gap(5).align(CenterX, CenterY))
    .children(|ui| {
      if ui.is_just_pressed(id) && can_afford {
        state.resource_pool.carbon -= cost_c;
        state.resource_pool.water -= cost_w;
        state.active_nodes.push(base);
      }

      if !can_afford {
        ui.element().width(grow!()).height(grow!())
          .floating(|f| f.attach_parent().z_index(10))
          .background_color((0u8, 0u8, 0u8, 150u8)).empty();
      }

      // Image placeholder
      ui.element().width(fixed!(64.0)).height(fixed!(64.0))
        .image(assets.test_image).empty();

      ui.text(tile.label(), |t| t.font_size(14).color(0xFFFFFF));

      // Yield/Trade info
      ui.element().width(grow!()).height(fit!())
        .layout(|l| l.direction(TopToBottom).align(CenterX, CenterY))
        .children(|ui| {
          if let Some(c) = trade.consumes_per_tick {
            ui.element().width(grow!())
              .layout(|l| l
                .direction(LeftToRight)
                .align(CenterX, CenterY)
                .gap(5)
              )
              .children(|ui| {
                ui.text(&format_resources_short(&c), |t| t.font_size(12).color(0xFFFFFF));
                ui.text("->", |t| t.font_size(12).color(0xFFFFFF));
              });
          }
          ui.text(&format_yield_short(&trade.yields_per_tick), |t| t.font_size(12).color(0xFFFFFF));
        });

      ui.element().width(grow!()).empty();

      // Cost info
      ui.element().width(grow!()).height(fit!())
        .layout(|l| l
          .direction(LeftToRight)
          .align(CenterX, CenterY)
          .gap(5)
        )
        .children(|ui| {
          ui.text(&format!("-{}C", cost_c as i32), |t| {
            let c = if state.resource_pool.carbon < cost_c { COLOR_RED } else { 0xAAAAAA };
            t.font_size(11).color(c)
          });
          ui.text(&format!("-{}H", cost_w as i32), |t| {
            let c = if state.resource_pool.water < cost_w { COLOR_RED } else { 0xAAAAAA };
            t.font_size(11).color(c)
          });
        });
    });
}

fn format_resources_short(r: &Resources) -> String {
  let mut parts = vec![];
  if r.carbon > 0.0 { parts.push(format!("-{}C", r.carbon as i32)); }
  if r.nitrogen > 0.0 { parts.push(format!("-{}N", r.nitrogen as i32)); }
  if r.phosphorus > 0.0 { parts.push(format!("-{}P", r.phosphorus as i32)); }
  if r.water > 0.0 { parts.push(format!("-{}H", r.water as i32)); }
  parts.join(" ")
}

fn format_yield_short(r: &Resources) -> String {
  let mut parts = vec![];
  if r.carbon > 0.0 { parts.push(format!("+{}C", r.carbon as i32)); }
  if r.nitrogen > 0.0 { parts.push(format!("+{}N", r.nitrogen as i32)); }
  if r.phosphorus > 0.0 { parts.push(format!("+{}P", r.phosphorus as i32)); }
  if r.water > 0.0 { parts.push(format!("+{}H", r.water as i32)); }
  parts.join(" ")
}

fn render_bottom_bar(ui: &mut Ui, state: &mut GameState) {
  ui.element().width(grow!()).height(fit!())
    .background_color(0x000000)
    .layout(|l| l.gap(10).padding(10))
    .children(|ui| {
      // Outstack button
      let out_id = ui.element().width(grow!()).height(fixed!(50.0))
        .background_color(0x333333)
        .corner_radius(4.0)
        .children(|ui| {
          ui.text("OUTSTACKED", |t| t.color(0xFFFFFF).font_size(16));
        });
      if ui.is_just_pressed(out_id) {
        state.is_overstacked_menu_opened = true;
      }

      // Spore button
      let can_afford = state.resource_pool.carbon >= SPORE_POINT_COSTS.carbon
        && state.resource_pool.nitrogen >= SPORE_POINT_COSTS.nitrogen
        && state.resource_pool.phosphorus >= SPORE_POINT_COSTS.phosphorus
        && state.resource_pool.water >= SPORE_POINT_COSTS.water;

      let spore_id = ui.element().width(grow!()).height(fixed!(50.0))
        .background_color(if can_afford { COLOR_PHOSPHORUS } else { 0x222222 })
        .corner_radius(4.0)
        .children(|ui| {
          ui.text("BUY SPORE", |t| t.color(if can_afford { 0x000000 } else { 0x555555 }).font_size(16));
        });
      if ui.is_just_pressed(spore_id) && can_afford {
        state.resource_pool -= SPORE_POINT_COSTS;
        state.spore_points += 1;
      }
    });
}

fn render_outstack_overlay(ui: &mut Ui, state: &mut GameState, assets: &Assets) {
  ui.element().width(grow!()).height(grow!())
    .floating(|f| f.attach_root().z_index(100))
    .background_color((0u8, 0u8, 0u8, 200u8))
    .layout(|l| l.align(CenterX, CenterY).padding(20))
    .children(|ui| {
      ui.element().width(fit!(max: 400.0)).height(fit!(max: 600.0))
        .background_color(COLOR_CARD_BG)
        .corner_radius(12.0)
        .layout(|l| l.direction(TopToBottom).padding(20).gap(15))
        .children(|ui| {
          ui.text("SELECT TILE TO REMOVE", |t| t.font_size(20).color(0xFFFFFF).alignment(CenterX));
          
          ui.element().width(grow!()).height(grow!(weight: 1.0))
            .overflow(|o| o.scroll_y())
            .layout(|l| l.direction(TopToBottom).gap(10))
            .children(|ui| {
              let mut to_remove = None;
              for (idx, node) in state.active_nodes.iter().enumerate() {
                let tile = node.get_current_tile_type(state.current_phase);
                let id = ui.element().width(grow!()).height(fixed!(60.0))
                  .background_color(0x2A2A2A)
                  .corner_radius(4.0)
                  .layout(|l| l.gap(10).padding(10).align(CenterX, CenterY))
                  .children(|ui| {
                    ui.element().width(fixed!(40.0)).height(fixed!(40.0)).image(assets.test_image).empty();
                    ui.text(tile.label(), |t| t.color(0xFFFFFF));
                    ui.element().width(grow!()).empty();
                    ui.text("REMOVE", |t| t.color(COLOR_RED));
                  });
                if ui.is_just_pressed(id) {
                  to_remove = Some(idx);
                }
              }
              if let Some(idx) = to_remove {
                state.active_nodes.remove(idx);
                state.is_overstacked_menu_opened = false;
              }
            });

          let cancel_id = ui.element().width(grow!()).height(fixed!(50.0))
            .background_color(0x333333)
            .corner_radius(4.0)
            .children(|ui| {
              ui.text("CANCEL", |t| t.color(0xFFFFFF).alignment(CenterX));
            });
          if ui.is_just_pressed(cancel_id) {
            state.is_overstacked_menu_opened = false;
          }
        });
    });
}

fn render_game_over(ui: &mut Ui, state: &GameState) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(30))
    .children(|ui| {
      ui.text("GAME OVER", |t| t.font_size(48).color(COLOR_PHOSPHORUS));
      ui.text(&format!("FINAL SCORE: {} SPORE POINTS", state.spore_points), |t| t.font_size(24).color(0xFFFFFF));
      
      ui.element().width(fit!()).height(fit!())
        .layout(|l| l.direction(TopToBottom).gap(10).align(CenterX, CenterY))
        .children(|ui| {
          ui.text("FINAL RESOURCES:", |t| t.color(0xAAAAAA));
          ui.text(&format!("{} Carbon", state.resource_pool.carbon as i32), |t| t.color(COLOR_CARBON));
          ui.text(&format!("{} Nitrogen", state.resource_pool.nitrogen as i32), |t| t.color(COLOR_NITROGEN));
          ui.text(&format!("{} Phosphorus", state.resource_pool.phosphorus as i32), |t| t.color(COLOR_PHOSPHORUS));
          ui.text(&format!("{} Water", state.resource_pool.water as i32), |t| t.color(COLOR_WATER));
        });
    });
}

