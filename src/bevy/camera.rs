//! Handle main camera

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
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
	static ref INITIAL_ROT: Quat = Quat::from_rotation_x(-45f32.to_radians());
}

impl CameraPlugin {
	/// Returns the default camera
	pub fn default() -> impl Bundle {
		let initial_pos = Vec3::new(0., CAMERA_HEIGHT, 0.);
		(
			Camera3dBundle {
				transform: Transform::from_translation(initial_pos),
				camera_3d: Camera3d {
					// gives the black background of space
					clear_color: ClearColorConfig::Custom(Color::BLACK),
					..default()
				},
				..default()
			},
			Rig::builder()
				.with(Position::new(Vec3::ZERO))
				.with(Rotation::new(*INITIAL_ROT))
				.with(Arm::new(Vec3::new(0., 50., 50.,)))
				// .with(
				// 	LookAt::new(Vec3::ZERO)
				// 		.tracking_predictive(false)
				// 		.tracking_smoothness(0.),
				// )
				.with(RotationArm::new(*INITIAL_ROT))
				// .with(Smooth::new_position(0.75).predictive(true))
				.build(),
			RaycastPickCamera::default(),
			MainCamera,
		)
	}
}

#[derive(Debug, Constructor)]
struct RotationArm {
	offset: Quat,
}

impl RigDriver for RotationArm {
	fn update(&mut self, params: bevy_dolly::dolly::rig::RigUpdateParams) -> Transform {
		Transform {
			rotation: params.parent.rotation.mul_quat(self.offset),
			translation: params.parent.translation,
			scale: Vec3::ONE,
		}
	}
}

/// Added to world in `PlayerPlugin` after player movement system
pub fn handle_camera_movement(
	player: Query<&Transform, (With<MainPlayer>, Without<MainCamera>)>,
	mut camera: Query<&mut Rig, (With<MainCamera>, Without<MainPlayer>)>,
) {
	let player = player.single();
	let mut rig = camera.single_mut();

	rig.driver_mut::<Position>().position = player.translation;
	rig.driver_mut::<Rotation>().rotation = player.rotation;
	// rig.driver_mut::<LookAt>().target = player.translation;
}
