use crate::models::*;
use ply_engine::prelude::*;

pub const COLOR_CARBON: u32 = 0xBDBDBD; // Gray-ish
pub const COLOR_NITROGEN: u32 = 0x4CAF50; // Green
pub const COLOR_PHOSPHORUS: u32 = 0xFFC107; // Amber
pub const COLOR_WATER: u32 = 0x2196F3; // Blue
pub const COLOR_RED: u32 = 0xF44336;
pub const COLOR_BG: u32 = 0x121212;
pub const COLOR_CARD_BG: u32 = 0x1E1E1E;

pub fn render_ui(ui: &mut Ui, mode: &mut GameMode) {
  ui.element().width(grow!()).height(grow!())
    .background_color(COLOR_BG)
    .children(|ui| {
      match mode {
        GameMode::StartSync { hold_accumulation } => {
          render_sync_screen(ui, "HOLD TO START SYNC", *hold_accumulation);
        }
        GameMode::Playing { state } => {
          render_dashboard(ui, state);
          if state.is_overstacked_menu_opened {
            render_outstack_overlay(ui, state);
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

fn render_dashboard(ui: &mut Ui, state: &mut GameState) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.direction(TopToBottom))
    .children(|ui| {
      render_top_bar(ui, state);

      ui.element().width(grow!()).height(grow!(weight: 1.0))
        .overflow(|o| o.scroll_y())
        .layout(|l| l.align(CenterX, CenterY).padding(20))
        .children(|ui| {
          render_grid(ui, state);
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
          resource_label(ui, "C", state.resource_pool.carbon, state.is_resource_missing.carbon && state.resource_pool.carbon < 0.5, COLOR_CARBON, (state.income_per_tick.carbon / 1000.0).min(1.0));
          resource_label(ui, "N", state.resource_pool.nitrogen, state.is_resource_missing.nitrogen && state.resource_pool.nitrogen < 0.5, COLOR_NITROGEN, (state.income_per_tick.nitrogen / 100.0).min(1.0));
          resource_label(ui, "P", state.resource_pool.phosphorus, state.is_resource_missing.phosphorus && state.resource_pool.phosphorus < 0.5, COLOR_PHOSPHORUS, (state.income_per_tick.phosphorus / 100.0).min(1.0));
          resource_label(ui, "H2O", state.resource_pool.water, state.is_resource_missing.water && state.resource_pool.water < 0.5, COLOR_WATER, (state.income_per_tick.water / 100.0).min(1.0));

          ui.element().width(grow!()).empty();
          ui.text(&format!("{} SP", state.spore_points), |t| t.font_size(18).color(0xFFFFFF));
        });
    });
}

/// excitement is a value between 0.0 and 1.0 that controls the wave effect of the number
fn resource_label(ui: &mut Ui, label: &str, value: f32, is_missing: bool, color: u32, excitement: f32) {
  let display_color = if is_missing { COLOR_RED } else { color };
  ui.element().width(fit!()).height(fit!())
    .layout(|l| l.gap(2))
    .children(|ui| {
      ui.text(&format!("{{pulse_f={:.2}_a={:.2}|{{wave_f={:.2}_a={:.2}|{}}}}}", excitement / 3.0, excitement / 10.0, excitement / 3.0, excitement / 10.0, value.floor() as i32), |t| t.font_size(16).color(display_color));
      ui.text(&format!("{}", label), |t| t.font_size(14).color(0xFFFFFF));
    });
}

fn render_grid(ui: &mut Ui, state: &mut GameState) {
  let scaling_factor = 1.0; // TODO: This should be based on screen size
  let available_bases = BaseTileType::base_types_by_phase(state.current_phase);

  let cols = if screen_width() > screen_height() && available_bases.len() != 4 { 3 } else { 2 };

  ui.element().contain(1.0)
    .layout(|l| l.direction(TopToBottom).gap((15.0 * scaling_factor) as u16).align(CenterX, CenterY))
    .children(|ui| {
      for (row_index, row) in available_bases.chunks(cols as usize).enumerate() {
        ui.element().width(grow!()).height(grow!())
          .layout(|l| l.gap((15.0 * scaling_factor) as u16))
          .children(|ui| {
            for (col_index, base) in row.iter().enumerate() {
              let button_index = row_index * cols as usize + col_index;
              render_tile_button(ui, state, *base, button_index, scaling_factor);
            }
          });
      }
    });
}

fn render_tile_button(ui: &mut Ui, state: &mut GameState, base: BaseTileType, button_index: usize, scaling_factor: f32) {
  let (amount, fraction, is_investing_current) = {
    let button = state
      .invest_button_data
      .get(button_index)
      .expect("Missing tile investment button data");
    (button.amount.clone(), button.fraction, button.is_investing)
  };
  let tile = base.get_current_tile_type(state.current_phase);

  let remainder = amount * (1.0 - fraction);
  let remainder_payable = state.resource_pool.minimum_fraction_fulfilled(&remainder).0;
  let total_payable = fraction + ((1.0 - fraction) * remainder_payable);
  let can_afford = total_payable == 1.0;

  let trade = tile.get_trade();
  let id = Id::new_index("tile_btn", base as u32);

  ui.element().width(grow!()).height(grow!())
    .id(id.clone())
    .background_color(COLOR_CARD_BG)
    .corner_radius(8.0)
    .border(|b| b.all(1).color(0x333333))
    .layout(|l| l.direction(TopToBottom).align(CenterX, CenterY))
    .children(|ui| {
      let pressed = ui.is_pressed(id.clone());
      let just_pressed = ui.is_just_pressed(id);
      let is_investing = if can_afford && ((pressed && is_investing_current) || just_pressed) {
        state.resource_pool -= remainder;
        state.active_nodes.push(base);
        let button = state.invest_button_data.get_mut(button_index).unwrap();
        button.fraction = 0.0;
        false
      } else {
        just_pressed || (pressed && is_investing_current)
      };

      ui.element().contain(1.0)
        .image(tile.icon()).empty();

      // ui.text(tile.label(), |t| t.font_size(14).color(0xFFFFFF));

      // // Yield/Trade info
      // ui.element().width(grow!()).height(fit!())
      //   .layout(|l| l.direction(TopToBottom).align(CenterX, CenterY))
      //   .children(|ui| {
      //     if let Some(c) = trade.consumes_per_tick {
      //       ui.element().width(grow!())
      //         .layout(|l| l
      //           .direction(LeftToRight)
      //           .align(CenterX, CenterY)
      //           .gap(5)
      //         )
      //         .children(|ui| {
      //           ui.text(&format_resources_short(&c), |t| t.font_size(12).color(0xFFFFFF));
      //           ui.text("->", |t| t.font_size(12).color(0xFFFFFF));
      //         });
      //     }
      //     ui.text(&format_yield_short(&trade.yields_per_tick), |t| t.font_size(12).color(0xFFFFFF));
      //   });

      ui.element().width(fixed!(75.0)).height(fixed!(16.0))
        .image(render_investment_bar(75.0, total_payable, fraction))
        .floating(|f| f.attach_parent().anchor((CenterX, CenterY), (CenterX, Bottom)))
        .corner_radius(10.0)
        .layout(|l| l.align(CenterX, CenterY))
        .border(|b| b.all(1).color(if can_afford { 0x016128 } else { 0xB01B2E }).position(Middle))
        .children(|ui| {
          ui.text(&format!("-{}C -{}H", amount.carbon as i32, amount.water as i32), |t| t.font_size(11).color(WHITE));
        });

      let button = state
        .invest_button_data
        .get_mut(button_index)
        .expect("Missing tile investment button data");
      button.is_investing = is_investing;
    });
}

fn render_investment_bar(width: f32, total_payable: f32, invested_fraction: f32) -> Texture2D {
  render_to_texture(width, 1.0, || {
    clear_background(BLACK);
    draw_rectangle(0.0, 0.0, width * total_payable, 1.0, if total_payable == 1.0 { Color::from(0x008F39).into() } else { Color::from(0xB01B2E).into() });
    draw_rectangle(0.0, 0.0, width * invested_fraction, 1.0, Color::from(0xD4AF37).into());
  })
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

      let spore_data = state.invest_button_data.last_mut().expect("Where did the button data go????");
      let spore_remainder = SPORE_POINT_COSTS * (1.0 - spore_data.fraction);
      let spore_remainder_payable = state.resource_pool.minimum_fraction_fulfilled(&spore_remainder).0;
      let spore_total_payable = spore_data.fraction + ((1.0 - spore_data.fraction) * spore_remainder_payable);

      let can_afford = spore_total_payable == 1.0;

      let spore_id = ui.element().width(grow!()).height(fixed!(50.0))
        .background_color(0x333333)
        .corner_radius(4.0)
        .layout(|l| l.direction(TopToBottom))
        .children(|ui| {
          ui.text("SPORE", |t| t.color(WHITE).font_size(16));
          ui.text(&format_resources_short(&SPORE_POINT_COSTS), |t| t
            .color(WHITE)
            .font_size(12)
          );
          ui.element().width(grow!()).height(grow!())
            .image(render_investment_bar(screen_width(), spore_total_payable, spore_data.fraction))
            .empty();
        });
      
      let pressed = ui.is_pressed(spore_id.clone());
      let just_pressed = ui.is_just_pressed(spore_id);

      spore_data.is_investing =
        if can_afford && ((pressed && spore_data.is_investing) || just_pressed) {
          state.resource_pool -= spore_remainder;
          state.spore_points += 1;
          spore_data.fraction = 0.0;
          false
        } else {
          just_pressed || (pressed && spore_data.is_investing)
        };
    });
}

fn render_outstack_overlay(ui: &mut Ui, state: &mut GameState) {
  ui.element().width(grow!()).height(grow!())
    .floating(|f| f.attach_root().z_index(100))
    .background_color((0u8, 0u8, 0u8, 200u8))
    .layout(|l| l.align(CenterX, CenterY).padding(20))
    .children(|ui| {
      // TODO: This should also be a grid
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
                    ui.element().width(fixed!(40.0)).height(fixed!(40.0)).image(tile.icon()).empty();
                    ui.text(tile.label(), |t| t.color(0xFFFFFF).font_size(16));
                    ui.element().width(grow!()).empty();
                    ui.text("REMOVE", |t| t.color(COLOR_RED).font_size(16));
                  });
                // TODO: It should be click to select, not click to remove. Anothter confimation button should be added. 
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
              ui.text("CANCEL", |t| t.color(0xFFFFFF).font_size(16));
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
          ui.text("FINAL RESOURCES:", |t| t.color(0xAAAAAA).font_size(16));
          ui.text(&format!("{} Carbon", state.resource_pool.carbon as i32), |t| t.color(COLOR_CARBON).font_size(14));
          ui.text(&format!("{} Nitrogen", state.resource_pool.nitrogen as i32), |t| t.color(COLOR_NITROGEN).font_size(14));
          ui.text(&format!("{} Phosphorus", state.resource_pool.phosphorus as i32), |t| t.color(COLOR_PHOSPHORUS).font_size(14));
          ui.text(&format!("{} Water", state.resource_pool.water as i32), |t| t.color(COLOR_WATER).font_size(14));
        });
    });
}

