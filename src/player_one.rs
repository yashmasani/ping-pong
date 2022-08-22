use bevy::prelude::*;

#[derive(Component,Clone, Copy, PartialEq, Eq)]
pub struct PlayerOne;

const MAX : f32 = 250.0;

pub fn player_one_move (mut keyboard_input: Res<Input<KeyCode>> , mut player_positions: Query<(&PlayerOne, &mut Transform)>) {
    for (_head, mut transform) in player_positions.iter_mut() {
        if transform.translation.y.abs() < MAX {
            if keyboard_input.pressed(KeyCode::W) {
                transform.translation.y += 2.0;
            } else if keyboard_input.pressed(KeyCode::S) {
                transform.translation.y -= 2.0;
            }
        }
    }
}
