use bevy::prelude::*;

#[path = "./ball.rs"] mod ball;
#[path = "./player_one.rs"] mod player_one;
use ball::Ball;
use player_one::PlayerOne;
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

pub fn ball_collide (
        mut ball: Query<(Option<&mut Direction>, &Transform)>, 
        player: Query<(Option<&PlayerOne>, &Transform)>,
    ) {
    let mut transform_player = &Transform::default();
    let mut transform_ball = Transform::default();
    for (dir, transform) in ball.iter() {
        if let Some(dir) = dir {
            transform_ball  = transform.clone();
        }
    }
    for (play, transform) in player.iter() {
        if let Some(play) = play {
            transform_player = transform;
            println!("{}", transform.translation.x);
        }
    }
    for (mut dir, _,) in ball.iter_mut() {
        if (*transform_player).translation.x == transform_ball.translation.x {
            if let Some(mut dir) = dir {
                dir.x = dir.x.abs();
            }
        }
    }
}
