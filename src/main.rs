use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle};

mod ball;
mod player_one;
mod direction;

use ball::Ball;
use direction::Direction;
use player_one::PlayerOne;

pub struct HelloPlugin;

const MAX : f32 = 250.0;

#[derive(Component)]
pub struct PlayerTwo; 

fn setup (mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { color: Color::Rgba { red: 0.00, green: 0.00, blue: 0.00, alpha: 1.00 }, 
            custom_size: Some(Vec2::new(30.0, 100.0)),
            ..default() },
        transform: Transform::from_translation(Vec3::new(-245.0, 00.0, 0.0)),
        ..default()
    }).insert(PlayerOne);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { color: Color::Rgba { red: 0.00, green: 0.00, blue: 0.00, alpha: 1.00 }, 
            custom_size: Some(Vec2::new(30.0, 100.0)),
            ..default() },
        transform: Transform::from_translation(Vec3::new(245.0, 00.0, 0.0)),
        ..default()
    }).insert(PlayerTwo);
    commands.spawn_bundle( MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..default()
    }).insert(Ball).insert(Direction {
        x: -1.0,
        y: -1.0,
    });
}


fn player_two_move (mut keyboard_input: Res<Input<KeyCode>> , mut player_positions: Query<(&PlayerTwo, &mut Transform)>) {
    for (_head, mut transform) in player_positions.iter_mut() {
        if transform.translation.y.abs() < MAX {
            if keyboard_input.pressed(KeyCode::Up) {
                transform.translation.y += 2.0;
            } else if keyboard_input.pressed(KeyCode::Down) {
                transform.translation.y -= 2.0;
            }
        }
    }
}

pub fn ball_collide (
        player_position: Query<&Transform, With<PlayerOne>>,
        mut dir_position: Query<(&Transform, &mut Direction), Without<PlayerOne>>,
    ) {
    if let Some(transform_player) = player_position.iter().next() {
        if let Some((trans_dir, mut dir)) = dir_position.iter_mut().next() {
            if transform_player.translation.x == trans_dir.translation.x {
                dir.x = dir.x.abs();
            }
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        println!("{}", "App is running");
        app.add_startup_system(setup);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Game".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system(player_one::player_one_move)
        .add_system(player_two_move)
        .add_system(direction::ball_move)
        .add_system(ball_collide)
        .run();
}
