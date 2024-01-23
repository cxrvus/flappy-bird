use std::time::Duration;
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use super::structs::*;


mod player {
	pub const WIDTH: f32 = 512.; //PX
	pub const HEIGHT: f32 = 256.; //PX
	pub const SCALE: f32 = 0.5;
	pub const ZPOS: f32 = 1.; //PX
	pub const WEIGHT: f32 = 10.;
	pub const GRAVITY: f32 = 20.;
	pub const FORCE: f32 = 10000.;
}

mod pipes {
	pub const HEIGHT: f32 = 600.; //PX
	pub const WIDTH: f32 = 196.; //PX
	pub const GAP: f32 = 200.; //PX between upper & lower
	pub const INTERVAL: f32 = 2.; //SECONDS between spawns
	pub const SPEED: f32 = 5.; //PX per frame
	pub const MAX_Y_OFFSET: f32 = 150.; //PX - max above/below screen center
	pub const ZPOS: f32 = 2.; //PX
}

mod env {
	pub const W_HEIGHT: f32 = 720.; //PX
	pub const W_WIDTH: f32 = 1280.; //PX
	pub const GROUND_HEIGHT: f32 = 50.; //PX
	pub const GROUND_ZPOS: f32 = 0.5; //PX
}


pub fn setup
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut timer: ResMut<PipeTimer>
) {
	timer.0 = Timer::new(Duration::from_secs_f32(pipes::INTERVAL), TimerMode::Repeating);

	commands.spawn(Camera2dBundle::default());

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/underwater.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(env::W_WIDTH, env::W_HEIGHT)),
			..default()
		},
		..default()
	})
	.insert(Background);
}


pub fn reset_background
(
	mut bg_transform: Query<&mut Transform, With<Background>>
) {
	let mut bg_transform = bg_transform.single_mut();
	bg_transform.translation = Vec3::default();
}


pub fn spawn_player
(
	mut commands: Commands,
	window: Query<&Window, With<bevy::window::PrimaryWindow>>,
	asset_server: Res<AssetServer>,
) {
	let window = window.single();
	let xpos = window.width() / -2. + player::WIDTH / 2.;

	commands.spawn(SpriteBundle {
		texture: asset_server.load("sprites/fish.png"),
		transform: Transform {
			translation: Vec3::new(xpos, 0.0, player::ZPOS),
			scale: Vec3::new(player::SCALE, player::SCALE, 1.0),
			..default()
		},
		..default()
	})
	.insert(Player)
	.insert(RigidBody::Dynamic)
	.insert(ExternalImpulse::default())
	.insert(LockedAxes::ROTATION_LOCKED_Z)
	.insert(LockedAxes::TRANSLATION_LOCKED_X)
	.insert(GravityScale(player::GRAVITY))
	.insert(Collider::cuboid(player::WIDTH / 2. * player::SCALE, player::HEIGHT / 2. * player::SCALE))
	.insert(ColliderMassProperties::Mass(player::WEIGHT))
	.insert(ActiveEvents::COLLISION_EVENTS)
	;
}


pub fn jump
(
	kb: Res<Input<KeyCode>>,
	mut impulse: Query<&mut ExternalImpulse, With<Player>>
) {
	if kb.just_pressed(KeyCode::Space) {
		if let Ok(mut impulse) = impulse.get_single_mut() {
			impulse.impulse = Vec2::new(0., player::FORCE);
		};
	}
}


pub fn collision_check
(
	mut collision_events: EventReader<CollisionEvent>,
	mut state: ResMut<NextState<GameState>>
) {
	if let Some(_) = collision_events.read().next() {
		state.set(GameState::GameOver);
	}
}


pub fn spawn_ground
(
	mut commands: Commands,
	window: Query<&Window, With<bevy::window::PrimaryWindow>>,
)
{
	let window = window.single();
	let ground_size = Vec2::new(window.width(), env::GROUND_HEIGHT);
	let ground_ypos = window.height() / -2. + env::GROUND_HEIGHT / 2.;
	commands.spawn(SpriteBundle {
		sprite: Sprite { color: Color::rgb(0., 0., 0.1), custom_size: Some(ground_size), ..default() },
		transform: Transform::from_translation(Vec3::new(0., ground_ypos, env::GROUND_ZPOS)),
		..default()
	})
	.insert(Collider::cuboid(ground_size.x / 2., ground_size.y / 4.))
	;
}


pub fn spawn_pipes
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut timer: ResMut<PipeTimer>
) {
	let texture = asset_server.load("sprites/pipe.png");
	let collider = Collider::cuboid(pipes::WIDTH / 2., pipes::HEIGHT / 2.);

	// let offset = -pipes::MAX_Y_OFFSET;
	let mut rng = thread_rng();
	let offset = rng.gen_range(-pipes::MAX_Y_OFFSET..=pipes::MAX_Y_OFFSET);
	let xpos = env::W_WIDTH / 2.;
	let ypos_lower = offset - (pipes::GAP / 2.);
	let ypos_upper = offset + (pipes::GAP / 2.);

	commands.spawn(SpriteBundle {
		texture: texture.clone(),
		transform: Transform::from_translation(Vec3::new(xpos, ypos_lower, pipes::ZPOS)),
		sprite: Sprite { anchor: Anchor::TopCenter, ..default() },
		..default()
	})
	.insert(Pipe)
	.insert(collider.clone())
	;

	commands.spawn(SpriteBundle {
		texture: texture.clone(),
		transform: Transform::from_translation(Vec3::new(xpos, ypos_upper, pipes::ZPOS)),
		sprite: Sprite { anchor: Anchor::BottomCenter, flip_y: true, ..default() },
		..default()
	})
	.insert(Pipe)
	.insert(collider.clone())
	;
}


// todo: move pipes
// todo: despawn pipes

