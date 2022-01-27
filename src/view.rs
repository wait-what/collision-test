use crate::State;
use macroquad::prelude::*;

pub fn render(state: &State) {
    // Draw player
    let player = state.player.position;
    draw_rectangle(player.x, player.y, player.w, player.h, RED);

    // Draw objects
    for object in &state.objects {
        draw_rectangle(object.x, object.y, object.w, object.h, GREEN);
    }
}
