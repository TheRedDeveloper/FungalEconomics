mod models;
mod engine;
mod ui;

use ply_engine::prelude::*;
use models::*;
use engine::*;
use ui::*;

fn window_conf() -> macroquad::conf::Conf {
  macroquad::conf::Conf {
    miniquad_conf: miniquad::conf::Conf {
      window_title: "Fungal Economics: Spore War".to_owned(),
      window_width: 400,
      window_height: 800,
      high_dpi: true,
      sample_count: 4,
      platform: miniquad::conf::Platform {
        webgl_version: miniquad::conf::WebGLVersion::WebGL2,
        ..Default::default()
      },
      ..Default::default()
    },
    draw_call_vertex_capacity: 100000,
    draw_call_index_capacity: 100000,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  static DEFAULT_FONT: FontAsset = FontAsset::Bytes { file_name: "lexend.ttf", data: include_bytes!("../assets/fonts/lexend.ttf") };
  
  let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

  let next_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/next.wav")).await.unwrap();
  let pause_sound = load_sound_from_bytes(include_bytes!("../assets/sounds/pause.wav")).await.unwrap();

  let mut mode = GameMode::StartSync { hold_accumulation: 0.0 };

  loop {
    clear_background(BLACK);

    let dt = get_frame_time();

    if let Some(effect) = update_game(&mut mode, dt) {
        match effect {
            SoundEffect::NextPhase => play_sound_once(&next_sound),
            SoundEffect::Pause => play_sound_once(&pause_sound),
        }
    }

    let mut ui = ply.begin();
    render_ui(&mut ui, &mut mode);
    ui.show(|_| {}).await;

    draw_fps();
    next_frame().await;
  }
}

