//! Handle main camera

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;

use super::player::MainPlayer;
use crate::utils::*; 

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, Dolly::<MainCamera>::update_active);
	}
}

#[derive(Component)]
pub struct MainCamera;

lazy_static::lazy_static! {
	static ref INITIAL_ROT: Quat = Quat::from_rotation_x(-90f32.to_radians());
}

impl CameraPlugin {
	/// Returns the default camera
	pub fn default() -> impl Bundle  {
		let initial_pos = Vec3::new(0., CAMERA_HEIGHT, 0.);
		(
			Camera3dBundle {
				transform: Transform::from_translation(initial_pos).with_rotation(*INITIAL_ROT),
				..default()
			},
			// Rig::builder()
			// 	.with(MovableLookAt::from_position_target(Vec3::ZERO))
			// 	.with(Arm::new(Vec3::new(0., CAMERA_HEIGHT, 5.)))
			// 	.build(),
			Rig::builder()
				.with(Position::new(Vec3::ZERO))
				// .with(Rotation::new(*INITIAL_ROT))
				.with(Smooth::new_position(0.75).predictive(true))
				.with(Arm::new(Vec3::new(0., CAMERA_HEIGHT, 0.)))
				// .with(
				// 	LookAt::new(Vec3::ZERO)
				// 		.tracking_predictive(true)
				// 		.tracking_smoothness(1.25),
				// )
				.with(Rotation::new(*INITIAL_ROT))
				.build(),
			RaycastPickCamera::default(),
			MainCamera,
		)
	}
}

/// Added to world in `PlayerPlugin` after player movement system
pub fn handle_camera_movement(
	player: Query<&Transform, (With<MainPlayer>, Without<MainCamera>)>,
	mut camera: Query<&mut Rig, (With<MainCamera>, Without<MainPlayer>)>,
) {
	let player = player.single();
	let mut rig = camera.single_mut();

	// rig
	// 	.driver_mut::<MovableLookAt>()
	// 	.set_position_target(player.translation, *INITIAL_ROT);

	rig.driver_mut::<Position>().position = player.translation + Vec3::Y;
	// rig.driver_mut::<LookAt>().target = player.translation;
}
