use bevy::prelude::*;
use bevy_dolly::prelude::*;

use super::{player::MainPlayer, utils::CAMERA_HEIGHT};

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl CameraPlugin {
	/// Returns the default camera
	pub fn default() -> (Camera3dBundle, Rig) {
		let initial_pos = Vec3::new(0., CAMERA_HEIGHT, 0.);
		let initial_rot = Quat::from_rotation_x(-90_f32.to_radians());
		(
			Camera3dBundle {
				transform: Transform::from_translation(initial_pos).with_rotation(initial_rot),
				..default()
			},
			// Rig::builder()
			// 	.with(MovableLookAt::from_position_target(Vec3::ZERO))
			// 	.build(),
			Rig::builder()
				.with(Position::new(Vec3::ZERO))
				.with(Rotation::new(initial_rot))
				.with(Smooth::new_position(1.25).predictive(true))
				.with(Arm::new(Vec3::new(0., CAMERA_HEIGHT, 5.)))
				.build(),
		)
	}
}

/// Added to world in `PlayerPlugin` after player movement system
pub fn handle_camera_movement(
	player: Query<&Transform, (With<MainPlayer>, Without<MainCamera>)>,
	mut camera: Query<&mut Rig, (With<MainCamera>, Without<MainPlayer>)>,
) {
	info!("Update camera system running");
	let player = player.single();
	let mut rig = camera.single_mut();

	info!("Updating camera");

	rig
		.driver_mut::<MovableLookAt>()
		.set_position_target(player.translation, player.rotation);
}
