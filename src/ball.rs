use bevy::prelude::*;

#[path = "./player_one.rs"] mod player_one;
#[path = "./direction.rs"] mod direction;
use direction::Direction;

const MAX : f32 = 250.0;

#[derive(Component)]
pub struct Ball;

pub fn ball_move (
    mut ball_position: Query<(&Ball, &mut Transform)>,
    mut dir: Query<&mut Direction>,
) { 
    for (ball, mut transform) in ball_position.iter_mut() {
        if transform.translation.y.abs() > MAX {
            dir.get_single_mut().unwrap().y *= -1.0;
        }
        if dir.get_single().is_ok() {
            transform.translation.x += dir.get_single().unwrap().x;
            transform.translation.y += dir.get_single().unwrap().y;
        }
    }
}

/*
 * pub fn ball_collide (
    mut set: ParamSet<( 
        Query<(&mut Ball, &mut Transform)>, 
        Query<&mut Transform, Without<player_one::PlayerOne>>
    )>,
    ) {
    let transform_player = set.p1().into_iter().next().unwrap();
    for (mut ball, transform_ball) in set.p0().iter_mut() {
        if transform_ball.translation.x == transform_player.translation.x && transform_ball.translation.y == transform_player.translation.y {
        ball.x = ball.x.abs();
         
        }
    }
}
*/
