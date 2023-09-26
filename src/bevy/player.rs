use crate::core::PlayerInventory;

use super::camera::{handle_camera_movement, MainCamera};
use crate::utils::*;
use bevy::prelude::*;
use std::ops::{Deref, Add, Div};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(
				Startup,
				(initial_spawn_player, |mut commands: Commands| {
					commands.insert_resource(PlayerInventory::default());
				}),
			)
			.add_systems(
				Update,
				(
					handle_player_movement,
					handle_camera_movement,
					handle_player_mined_px,
				),
			);
	}
}

const PLAYER_HEIGHT: f32 = 25.;

#[derive(Component)]
pub struct MainPlayer;

fn initial_spawn_player(mut commands: Commands, (mut meshs, mut mats, _): MMA) {
	info!("Spawning player");
	commands.spawn(
		(
			PbrBundle {
				material: mats.add(Color::SILVER.into()),
				transform: Transform::from_xyz(0., PLAYER_HEIGHT, 0.),
				mesh: meshs.add(shape::Box::new(2. * PIXEL_SIZE, 2. * PIXEL_SIZE, 2. * PIXEL_SIZE).into()),
				..default()
			},
			MainPlayer,
		)
			.named("Main Player")
			.physics_dynamic()
			.physics_collider_ball(10.)
			.physics_zero_force()
			.physics_zero_velocity(),
	);
}

impl MainPlayer {
	const MOVE_FACTOR: f32 = 5_000_000.;
	const TURN_FACTOR: f32 = MainPlayer::MOVE_FACTOR / 5.;

	const MAX_LINEAR_VELOCITY: f32 = 10.;
	const MAX_ANGULAR_VELOCITY: f32 = 10.;
}

fn handle_player_movement(
	mut player: Query<(&mut ExternalForce, &Transform, &Velocity), With<MainPlayer>>,
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	let (mut player, transform, velocity) = player.single_mut();

	// movement
	let mut movement_force = Vec3::ZERO;
	let mut torque = Vec3::ZERO;
	let forward = transform.forward().normalize();
	let up = transform.up().normalize();
	if keyboard_input.pressed(KeyCode::W) {
		// forward
		movement_force += forward;
	}
	if keyboard_input.pressed(KeyCode::S) {
		// backwards
		movement_force -= forward;
	}
	if keyboard_input.pressed(KeyCode::A) {
		// left
		torque += up;
	}
	if keyboard_input.pressed(KeyCode::D) {
		// right
		torque -= up;
	}
	if keyboard_input.pressed(KeyCode::Space) {
		// tilt up
		torque += forward.cross(up).normalize();
	}
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		// tilt down
		torque -= forward.cross(up).normalize();
	}

	if movement_force == Vec3::ZERO {
		player.force = Vec3::ZERO;
	} else {
		player.force =
			movement_force.normalize() * MainPlayer::MOVE_FACTOR * time.delta_seconds_f64() as f32;

		// limit velocity
		let current_velocity = velocity.linvel;

		let forward_factor = player
			.force
			.normalize()
			.dot(current_velocity.normalize())

			.add(1.)
			.div(2.);

		info!(
			"len = {} Forward factor: {}",
			current_velocity.length(),
			forward_factor
		);

		if current_velocity.length() > MainPlayer::MAX_LINEAR_VELOCITY {
			player.force *= 1. - forward_factor;
		}
	}
	if torque == Vec3::ZERO {
		player.torque = Vec3::ZERO;
	} else {
		player.torque = torque.normalize() * MainPlayer::TURN_FACTOR * time.delta_seconds_f64() as f32;

		// limit angular velocity
		if velocity.angvel.length() > MainPlayer::MAX_ANGULAR_VELOCITY {
			player.torque = Vec3::ZERO;
		}
	}

	// #[cfg(feature = "debugging")]
	// info!(
	// 	"Player force: {:?}, torque: {:?}, velocity: {:?}",
	// 	player.force, player.torque, velocity
	// );

	// #[cfg(feature = "debugging")]
	// info!("Player linear velocity (len = {}): {:?}", velocity.linvel.length(), velocity.linvel);
}

fn handle_player_mined_px(
	mut e: EventReader<PlayerMinedPixel>,
	mut inventory: ResMut<PlayerInventory>,
) {
	for px in e.iter().map(|p| p.deref()) {
		info!("Player mined pixel: {:?}", px);
		inventory[px.variant] += px.collectable.as_ref().unwrap().amount_multiplier as u32;
	}
}
