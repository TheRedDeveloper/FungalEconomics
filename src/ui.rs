use crate::models::*;
use ply_engine::prelude::*;

pub const COLOR_CARBON: u32 = 0xBDBDBD; // Gray-ish
pub const COLOR_NITROGEN: u32 = 0x4CAF50; // Green
pub const COLOR_PHOSPHORUS: u32 = 0xFFC107; // Amber
pub const COLOR_WATER: u32 = 0x2196F3; // Blue
pub const COLOR_RED: u32 = 0xF44336;
pub const COLOR_BG: u32 = 0x121212;

pub fn render_ui(ui: &mut Ui, mode: &mut GameMode) {
  let scaling_factor = ((screen_width() / 1080.0).min(screen_height() / 1920.0) * 3.0).max(0.5);

  ui.element().width(grow!()).height(grow!())
    .background_color(COLOR_BG)
    .children(|ui| {
      match mode {
        GameMode::StartSync { hold_accumulation } => {
          render_sync_screen(ui, "ZUM SYNCHRONISIEREN HALTEN", *hold_accumulation, scaling_factor);
        }
        GameMode::Playing { state } => {
          render_dashboard(ui, state, scaling_factor);
          if state.overstacked_menu.is_some() {
            render_outstack_overlay(ui, state, scaling_factor);
          }
        }
        GameMode::TransitionSync { state, hold_accumulator } => {
          render_sync_screen(ui, &format!("PHASE {}: ZUM SYNCHRONISIEREN HALTEN", state.current_phase), *hold_accumulator, scaling_factor);
        }
        GameMode::GameOver { state } => {
          render_game_over(ui, state, scaling_factor);
        }
      }
    });
}

fn render_sync_screen(ui: &mut Ui, message: &str, progress: f32, scaling_factor: f32) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap(20))
    .children(|ui| {
      ui.text(message, |t| t.font_size((24.0 * scaling_factor) as u16).color(0xFFFFFF));
      
      let p = (progress / SYNC_HOLD_TIME).clamp(0.0, 1.0);
      ui.element().width(fixed!(300.0 * scaling_factor)).height(fixed!(20.0 * scaling_factor))
        .background_color(0x333333)
        .children(|ui| {
          ui.element().width(fixed!(300.0 * p * scaling_factor)).height(grow!())
            .background_color(COLOR_WATER).empty();
        });

      ui.text("Alle Spieler müssen gleichzeitig halten", |t| t.font_size((14.0 * scaling_factor) as u16).color(0xAAAAAA));
    });
}

fn render_dashboard(ui: &mut Ui, state: &mut GameState, scaling_factor: f32) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.direction(TopToBottom))
    .children(|ui| {
      render_top_bar(ui, state, scaling_factor);

      ui.element().width(grow!()).height(grow!())
        .layout(|l| l.align(CenterX, CenterY).padding(20 * scaling_factor as u16))
        .children(|ui| {
          render_grid(ui, state, scaling_factor);
        });

      render_bottom_bar(ui, state, scaling_factor);
    });
}

fn render_top_bar(ui: &mut Ui, state: &mut GameState, scaling_factor: f32) {
  ui.element().width(grow!()).height(fit!())
    .background_color(0x000000)
    .layout(|l| l.direction(TopToBottom).padding(10).gap(5))
    .children(|ui| {
      // Title and Timer
      ui.element().width(grow!()).height(fit!())
        .children(|ui| {
          let mins = (state.phase_timer as i32) / 60;
          let secs = (state.phase_timer as i32) % 60;
          ui.text(&format!("PHASE {} • {:02}:{:02}", state.current_phase, mins, secs), |t| t.font_size((18.0 * scaling_factor) as u16).color(0xFFFFFF));
          ui.element().width(grow!()).empty();
          if !state.change_log.is_empty() {
            ui.element().id("undo_btn")
              .background_color(0x333333)
              .children(|ui| {
                let height = 18.0 * scaling_factor;
                ui.element().width(fixed!(height)).height(fixed!(height)).image(&UNDO_IMAGE).empty();
                ui.text(&format!("{}", state.change_log.last().unwrap().label()), |t| t.font_size((18.0 * scaling_factor) as u16).color(0xFFFFFF));
              });
            
            if ui.is_just_pressed("undo_btn") { state.undo_last_change(); }
          }
        });

      // Resources
      ui.element().width(grow!()).height(fit!())
        .layout(|l| l.gap(15))
        .children(|ui| {
          resource_label(ui, "C", state.resource_pool.carbon, state.is_resource_missing.carbon && state.resource_pool.carbon < 0.5, COLOR_CARBON, (state.income_per_tick.carbon / 1000.0).min(1.0), scaling_factor);
          resource_label(ui, "N", state.resource_pool.nitrogen, state.is_resource_missing.nitrogen && state.resource_pool.nitrogen < 0.5, COLOR_NITROGEN, (state.income_per_tick.nitrogen / 100.0).min(1.0), scaling_factor);
          resource_label(ui, "P", state.resource_pool.phosphorus, state.is_resource_missing.phosphorus && state.resource_pool.phosphorus < 0.5, COLOR_PHOSPHORUS, (state.income_per_tick.phosphorus / 100.0).min(1.0), scaling_factor);
          resource_label(ui, "H₂O", state.resource_pool.water, state.is_resource_missing.water && state.resource_pool.water < 0.5, COLOR_WATER, (state.income_per_tick.water / 100.0).min(1.0), scaling_factor);

          ui.element().width(grow!()).empty();
          ui.text(&format!("{} SP", state.spore_points), |t| t.font_size((18.0 * scaling_factor) as u16).color(0xFFFFFF));
        });
    });
}

/// excitement is a value between 0.0 and 1.0 that controls the wave effect of the number
fn resource_label(ui: &mut Ui, label: &str, value: f32, is_missing: bool, color: u32, excitement: f32, scaling_factor: f32) {
  let display_color = if is_missing { COLOR_RED } else { color };
  ui.element().width(fit!()).height(fit!())
    .layout(|l| l.gap(2))
    .children(|ui| {
      ui.text(&format!("{{pulse_f={:.2}_a={:.2}|{{wave_f={:.2}_a={:.2}|{}}}}}", excitement / 3.0, excitement / 10.0, excitement / 3.0, excitement / 10.0, value.floor() as i32), |t| t.font_size((16.0 * scaling_factor) as u16).color(display_color));
      ui.text(&format!("{}", label), |t| t.font_size((14.0 * scaling_factor) as u16).color(0xFFFFFF));
    });
}

fn render_grid(ui: &mut Ui, state: &mut GameState, scaling_factor: f32) {
  let available_bases = BaseTileType::base_types_by_phase(state.current_phase);

  let cols = if screen_width() > screen_height() && available_bases.len() != 4 { 3 } else { 2 };
  let rows = (available_bases.len() as f32 / cols as f32).ceil() as usize;

  ui.element().contain(cols as f32/rows as f32)
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

  let id = Id::new_index("tile_btn", base as u32);

  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.direction(TopToBottom).align(CenterX, CenterY))
    .children(|ui| {
      let pressed = ui.is_pressed(id.clone());
      let just_pressed = ui.is_just_pressed(id.clone());
      let stacking_but_not_featured = state.stack_mode && !state.active_nodes.contains(&base);
      let is_investing = if stacking_but_not_featured {
        false
      } else if
        can_afford &&
        ((pressed && is_investing_current) || just_pressed)
      {
        state.resource_pool -= remainder;
        let button = state.invest_button_data.get_mut(button_index).unwrap();
        button.fraction = 0.0;
        if state.stack_mode {
          state.change_log.push(Change::Stack(base));
          state.stack_mode = false;
        } else {
          state.active_nodes.push(base);
          state.change_log.push(Change::Add(base));
        }
        false
      } else {
        just_pressed || (pressed && is_investing_current)
      };

      let graphic = if state.stack_mode {
        tile.graphic_without_yield()
      } else {
        tile.graphic_with_yield()
      };
      ui.element().contain(13.0/15.0)
        .image(graphic)
        .background_color(if stacking_but_not_featured { 0x555555 } else { 0xFFFFFF })
        .id(id)
        .children(|ui| {
          ui.element().width(fixed!(75.0 * scaling_factor)).height(fixed!(16.0 * scaling_factor))
            .image(render_investment_bar(75.0 * scaling_factor, total_payable, fraction))
            .floating(|f| f.attach_parent().anchor((CenterX, CenterY), (CenterX, Bottom)).offset((0.0, -50.0*scaling_factor)))
            .corner_radius(10.0 * scaling_factor)
            .layout(|l| l.align(CenterX, CenterY))
            .border(|b| b.all((1.0 * scaling_factor).min(1.0) as u16).color(if can_afford { 0x016128 } else { 0xB01B2E }).position(Middle))
            .children(|ui| {
              ui.text(&format!("-{}C -{}H", amount.carbon as i32, amount.water as i32), |t| t.font_size((11.0 * scaling_factor) as u16).color(WHITE));
            });
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

fn render_bottom_bar(ui: &mut Ui, state: &mut GameState, scaling_factor: f32) {
  ui.element().width(grow!()).height(fit!())
    .background_color(0x000000)
    .layout(|l| l.gap((10.0 * scaling_factor) as u16).padding((10.0 * scaling_factor) as u16))
    .children(|ui| {
      // Outstack button
      let out_id = ui.element().width(grow!()).height(fixed!(50.0 * scaling_factor))
        .background_color(0x333333)
        .corner_radius(4.0 * scaling_factor)
        .layout(|l| l.align(CenterX, CenterY))
        .children(|ui| {
          ui.text("ÜBERSTAPELN", |t| t.color(0xFFFFFF).font_size((16.0 * scaling_factor) as u16));
        });
      if ui.is_just_pressed(out_id) {
        state.overstacked_menu = Some(None);
      }

      // Stack-mode button
      let bg = if state.stack_mode { COLOR_RED } else { 0x333333 };
      let stack_id = ui.element().width(fixed!(50.0 * scaling_factor)).height(fixed!(50.0 * scaling_factor))
        .background_color(bg)
        .corner_radius(10.0 * scaling_factor)
        .layout(|l| l.align(CenterX, CenterY).padding((10.0 * scaling_factor) as u16))
        .children(|ui| {
          ui.element().width(grow!()).height(grow!())
            .image(&STACK_IMAGE)
            .empty();
        });
      if ui.is_just_pressed(stack_id) {
        state.stack_mode = !state.stack_mode;
      }

      // Spore button
      let spore_data = state.invest_button_data.last_mut().expect("Where did the button data go????");
      let spore_remainder = SPORE_POINT_COSTS * (1.0 - spore_data.fraction);
      let spore_remainder_payable = state.resource_pool.minimum_fraction_fulfilled(&spore_remainder).0;
      let spore_total_payable = spore_data.fraction + ((1.0 - spore_data.fraction) * spore_remainder_payable);

      let can_afford = spore_total_payable == 1.0;

      let spore_id = ui.element().width(grow!()).height(fixed!(50.0 * scaling_factor))
        .background_color(0x333333)
        .corner_radius(4.0 * scaling_factor)
        .layout(|l| l.align(CenterX, CenterY))
        .children(|ui| {
          ui.text("SPOREN", |t| t.color(WHITE).font_size((16.0 * scaling_factor) as u16));
          ui.element().width(fixed!(screen_width()*0.3)).height(fixed!(16.0 * scaling_factor))
            .floating(|f| f.attach_parent().anchor((CenterX, CenterY), (CenterX, Bottom)))
            .image(render_investment_bar(screen_width(), spore_total_payable, spore_data.fraction))
            .corner_radius(10.0 * scaling_factor)
            .layout(|l| l.align(CenterX, CenterY))
            .border(|b| b.all((1.0 * scaling_factor).min(1.0) as u16).color(if can_afford { 0x016128 } else { 0xB01B2E }).position(Middle))
            .children(|ui| {
              ui.text(&format_resources_short(&SPORE_POINT_COSTS), |t| t
                .color(WHITE)
                .font_size((10.0 * scaling_factor) as u16)
              );
            });
        });
      
      let pressed = ui.is_pressed(spore_id.clone());
      let just_pressed = ui.is_just_pressed(spore_id);

      spore_data.is_investing =
        if can_afford && ((pressed && spore_data.is_investing) || just_pressed) {
          state.resource_pool -= spore_remainder;
          state.spore_points += 1;
          spore_data.fraction = 0.0;
          state.change_log.push(Change::Spore);
          false
        } else {
          just_pressed || (pressed && spore_data.is_investing)
        };
    });
}

fn render_outstack_overlay(ui: &mut Ui, state: &mut GameState, scaling_factor: f32) {
  let overlay_width = screen_width() * 0.8;
  let overlay_height = screen_height() * 0.8;
  let available_bases = BaseTileType::base_types_by_phase(state.current_phase);
  let cols = if overlay_width > overlay_height && available_bases.len() != 4 { 3 } else { 2 };

  ui.element().width(grow!()).height(grow!())
    .floating(|f| f.attach_root().z_index(100))
    .layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
      ui.element().width(fixed!(overlay_width)).height(fixed!(overlay_height))
        .background_color((18, 18, 18, 200))
        .corner_radius(12.0 * scaling_factor)
        .layout(|l| l.direction(TopToBottom).gap((16.0 * scaling_factor) as u16).padding((18.0 * scaling_factor) as u16))
        .children(|ui| {
          ui.text("FELD ZUM ÜBERNEHMEN AUSWÄHLEN", |t| t.font_size((22.0 * scaling_factor) as u16).color(0xFFFFFF));

          ui.element().width(grow!()).height(grow!())
            .layout(|l| l.direction(TopToBottom).gap((15.0 * scaling_factor) as u16).align(CenterX, CenterY))
            .children(|ui| {
              for (row_index, row) in available_bases.chunks(cols as usize).enumerate() {
                ui.element().width(grow!()).height(grow!())
                  .layout(|l| l.gap((15.0 * scaling_factor) as u16).align(CenterX, CenterY))
                  .children(|ui| {
                    for (col_index, base) in row.iter().enumerate() {
                      let button_index = row_index * cols as usize + col_index;
                      let tile = base.get_current_tile_type(state.current_phase);
                      let is_availiable = state.active_nodes.contains(base);
                      let is_selected = state.overstacked_menu == Some(Some(*base));
                      let id = Id::new_index("outstack_tile", button_index as u32);

                      let just_pressed = ui.is_just_pressed(Id::new_index("outstack_tile", button_index as u32));
                      if just_pressed {
                        if is_selected {
                          state.overstacked_menu = Some(None);
                        } else if is_availiable {
                          state.overstacked_menu = Some(Some(*base));
                        }
                      }

                      let bg = if is_availiable { 0xFFFFFF } else { 0x555555 }; 

                      ui.element().contain(13.0/15.0)
                        .id(id.clone())
                        .corner_radius(10.0 * scaling_factor)
                        .background_color(bg)
                        .image(tile.graphic_with_yield())
                        .empty();

                      // HACK: Draw a selection window
                      if is_selected {
                        if let Some(bb) = ui.bounding_box(id.clone()) {
                          ui.element().width(fixed!(bb.width+10.0 * scaling_factor)).height(fixed!(bb.height+10.0 * scaling_factor))
                            .floating(|f| f.attach_id(id).anchor((CenterX, CenterY), (CenterX, CenterY)).z_index(100).passthrough())
                            .background_color((0xFF, 0xFF, 0x00, 0x40))
                            .corner_radius(15.0 * scaling_factor)
                            .border(|b| b.all((2.0 * scaling_factor) as u16).color(0xFFFF00).position(Middle))
                            .empty();
                        }
                      }
                    }
                  });
              }
            });

          ui.element().width(grow!()).height(fit!())
            .layout(|l| l.gap((12.0 * scaling_factor) as u16))
            .children(|ui| {
              let is_selected = matches!(state.overstacked_menu, Some(Some(_)));
              let remove_color = if is_selected { COLOR_RED } else { 0x222222 };
              let cancel_color = 0x444444;

              let remove_id = ui.element().width(grow!()).height(fixed!(48.0 * scaling_factor))
                .background_color(remove_color)
                .corner_radius(6.0 * scaling_factor)
                .layout(|l| l.align(CenterX, CenterY))
                .children(|ui| {
                  ui.text("ÜBERNEHMEN", |t| t.color(0xFFFFFF).font_size((16.0 * scaling_factor) as u16));
                });

              if is_selected && ui.is_just_pressed(remove_id) {
                let selected_base = state.overstacked_menu.unwrap().unwrap();
                if let Some(idx) = state.active_nodes.iter().position(|&b| b == selected_base) {
                  state.active_nodes.remove(idx);
                  state.change_log.push(Change::Overtake(selected_base));
                  state.overstacked_menu = None;
                }
              }

              let cancel_id = ui.element().width(grow!()).height(fixed!(48.0 * scaling_factor))
                .background_color(cancel_color)
                .corner_radius(6.0 * scaling_factor)
                .layout(|l| l.align(CenterX, CenterY))
                .children(|ui| {
                  ui.text("ABBRECHEN", |t| t.color(0xFFFFFF).font_size((16.0 * scaling_factor) as u16));
                });

              if ui.is_just_pressed(cancel_id) {
                state.overstacked_menu = None;
              }
            });
        });
    });
}

fn render_game_over(ui: &mut Ui, state: &GameState, scaling_factor: f32) {
  ui.element().width(grow!()).height(grow!())
    .layout(|l| l.align(CenterX, CenterY).direction(TopToBottom).gap((30.0 * scaling_factor) as u16))
    .children(|ui| {
      ui.text("SPIEL VORBEI", |t| t.font_size((48.0 * scaling_factor) as u16).color(COLOR_PHOSPHORUS));
      ui.text(&format!("ENDSTAND: {} SPORENPUNKTE", state.spore_points), |t| t.font_size((24.0 * scaling_factor) as u16).color(0xFFFFFF));
      
      ui.element().width(fit!()).height(fit!())
        .layout(|l| l.direction(TopToBottom).gap(10).align(CenterX, CenterY))
        .children(|ui| {
          ui.text("ENDRESSOURCEN:", |t| t.color(0xAAAAAA).font_size((16.0 * scaling_factor) as u16));
          ui.text(&format!("{} Kohlenstoff", state.resource_pool.carbon as i32), |t| t.color(COLOR_CARBON).font_size((14.0 * scaling_factor) as u16));
          ui.text(&format!("{} Stickstoff", state.resource_pool.nitrogen as i32), |t| t.color(COLOR_NITROGEN).font_size((14.0 * scaling_factor) as u16));
          ui.text(&format!("{} Phosphor", state.resource_pool.phosphorus as i32), |t| t.color(COLOR_PHOSPHORUS).font_size((14.0 * scaling_factor) as u16));
          ui.text(&format!("{} Wasser", state.resource_pool.water as i32), |t| t.color(COLOR_WATER).font_size((14.0 * scaling_factor) as u16));
        });
    });
}

