use crate::core::PlayerInventory;

use super::camera::{handle_camera_movement, MainCamera};
use bevy::prelude::*;
use crate::utils::*;
use std::ops::Deref;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, (initial_spawn_player, |mut commands: Commands| {
				commands.insert_resource(PlayerInventory::default());
			}))
			.add_systems(Update, (handle_player_movement, handle_camera_movement, handle_player_mined_px));
	}
}

const PLAYER_HEIGHT: f32 = 25.;

#[derive(Component)]
pub struct MainPlayer;

fn initial_spawn_player(mut commands: Commands, (mut meshs, mut mats, _): MMA) {
	info!("Spawning player");
	commands.spawn((
		PbrBundle {
			material: mats.add(Color::SILVER.into()),
			transform: Transform::from_xyz(0., PLAYER_HEIGHT, 0.),
			mesh: meshs.add(shape::Box::new(2. * PIXEL_SIZE, 2. * PIXEL_SIZE, 2. * PIXEL_SIZE).into()),
			..default()
		},
		MainPlayer,
		Name::from("Main Player"),
	));
}

fn handle_player_movement(
	mut player: Query<&mut Transform, (With<MainPlayer>, Without<MainCamera>)>,
	// mut camera: Query<&mut Transform, (With<MainCamera>, Without<MainPlayer>)>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	let mut player = player.single_mut();
	// let mut camera = camera.single_mut();

	let mut translation = Vec3::ZERO;
	if keyboard_input.pressed(KeyCode::W) {
		translation -= Vec3::Z;
	}
	if keyboard_input.pressed(KeyCode::S) {
		translation += Vec3::Z;
	}
	if keyboard_input.pressed(KeyCode::A) {
		translation -= Vec3::X;
	}
	if keyboard_input.pressed(KeyCode::D) {
		translation += Vec3::X;
	}
	#[cfg(feature = "dev")]
	if keyboard_input.pressed(KeyCode::Space) {
		translation += Vec3::Y;
	}
	#[cfg(feature = "dev")]
	if keyboard_input.pressed(KeyCode::ShiftLeft) {
		translation -= Vec3::Y;
	}

	if translation != Vec3::ZERO {
		let translation = translation.normalize() * 2.;
		player.translation += translation;
		// camera.translation += translation;
	}
}

fn handle_player_mined_px(mut e: EventReader<PlayerMinedPixel>, mut inventory: ResMut<PlayerInventory>) {
	for px in e.iter().map(|p| p.deref()) {
		info!("Player mined pixel: {:?}", px);
		inventory[px.variant] += px.player_mineable.as_ref().unwrap().amount_multiplier as u32;
	}
}