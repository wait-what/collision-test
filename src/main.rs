use macroquad::prelude::*;
use std::time::Instant;

pub mod state;
mod view;

use state::State;

fn window_conf() -> Conf {
    Conf {
        window_title: "Collision test".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();

    let mut last_frame = Instant::now();
    loop {
        let now = Instant::now();
        let delta = now.duration_since(last_frame).as_secs_f32();

        state.update(delta);
        view::render(&state);

        next_frame().await;

        last_frame = now;
    }
}
