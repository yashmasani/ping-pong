use bevy::prelude::*;

const MAX : f32 = 250.0;

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32
}

pub fn ball_move (
    mut ball_position: Query<(&mut Transform, &mut Direction)>,
) { 
    for (mut transform, mut dir_position) in ball_position.iter_mut() {
        if transform.translation.y.abs() > MAX {
            dir_position.y *= -1.0;
        }
        transform.translation.x += dir_position.x;
        transform.translation.y += dir_position.y;
    }
}

