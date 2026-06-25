use crate::models::*;
use ply_engine::prelude::*;

pub fn update_game(mode: &mut GameMode, dt: f32) -> Option<SoundEffect> {
  match mode {
    GameMode::StartSync { hold_accumulation } => {
      if is_mouse_button_down(MouseButton::Left) {
        *hold_accumulation += dt;
        if *hold_accumulation >= SYNC_HOLD_TIME {
          *mode = GameMode::Playing { state: GameState::new() };
          return Some(SoundEffect::NextPhase);
        }
      } else {
        *hold_accumulation = (*hold_accumulation - dt * 10.0).max(0.0);
      }
    }
    GameMode::Playing { state } => {
      if state.is_paused {
        return None;
      }

      state.phase_timer -= dt;
      if state.phase_timer <= 0.0 {
        if state.current_phase >= 5 {
          *mode = GameMode::GameOver { state: state.clone() };
          return Some(SoundEffect::Pause);
        } else {
          *mode = GameMode::TransitionSync {
            state: state.clone(),
            hold_accumulator: 0.0,
          };
          return Some(SoundEffect::Pause);
        }
      }

      process_metabolism(state, dt);
    }
    GameMode::TransitionSync { state, hold_accumulator } => {
      if is_mouse_button_down(MouseButton::Left) {
        *hold_accumulator += dt;
        if *hold_accumulator >= SYNC_HOLD_TIME {
          state.current_phase += 1;
          state.reset_button_data();
          state.phase_timer = PHASE_LENGTH;
          *mode = GameMode::Playing { state: state.clone() };
          return Some(SoundEffect::NextPhase);
        }
      } else {
        *hold_accumulator = (*hold_accumulator - dt * 10.0).max(0.0);
      }
    }
    GameMode::GameOver { .. } => {}
  }
  None
}

fn process_metabolism(state: &mut GameState, dt: f32) {
  let scale = dt / TICK_LENGTH;
  state.is_resource_missing = IsResourceMissing::default();
  state.resource_pool += BASE_INCOME * scale;
  state.income_per_tick = BASE_INCOME;

  for button in &mut state.invest_button_data {
    if button.is_investing {
      let frac_remaining = 1.0 - button.fraction;
      let remainder = button.amount * frac_remaining;
      let (frac, missing) = state.resource_pool.minimum_fraction_fulfilled(&remainder);
      state.is_resource_missing |= missing;
      button.fraction += frac_remaining * frac;
      state.resource_pool -= remainder * frac;
    } else {
      // drain 100% of the fraction over DRAIN_TIME seconds
      let drain_amount = (dt / DRAIN_TIME).min(button.fraction);
      button.fraction -= drain_amount;
      state.resource_pool += button.amount * drain_amount;
    }
  }

  for node_base in &state.active_nodes {
    let tile_type = node_base.get_current_tile_type(state.current_phase);
    let trade = tile_type.get_trade();

    let fraction = if let Some(consumes) = trade.consumes_per_tick {
      let actual_consumptions = consumes * scale;
      let (frac, missing) = state.resource_pool.minimum_fraction_fulfilled(&actual_consumptions);
      state.is_resource_missing |= missing;
      
      state.resource_pool -= actual_consumptions * frac;
      state.income_per_tick -= consumes * frac;
      frac
    } else {
      1.0
    };

    state.resource_pool += trade.yields_per_tick * scale * fraction;
    state.income_per_tick += trade.yields_per_tick * fraction;

    for button in &mut state.invest_button_data {
      if button.is_investing {
        let frac_remaining = 1.0 - button.fraction;
        let remainder = button.amount * frac_remaining;
        let (frac, missing) = state.resource_pool.minimum_fraction_fulfilled(&remainder);
        state.is_resource_missing |= missing;
        button.fraction += frac_remaining * frac;
        state.resource_pool -= remainder * frac;
      }
    }
  }
}

pub enum SoundEffect {
  NextPhase,
  Pause,
}
