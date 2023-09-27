//! Handle main camera

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_dolly::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;

use super::player::MainPlayer;
use crate::utils::*;

mod orbit;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(Dolly::<MainCamera>::update_active, orbit::orbit_camera),
		);
	}
}

#[derive(Component)]
pub struct MainCamera;

lazy_static::lazy_static! {
	static ref INITIAL_ROT: Quat = Quat::from_rotation_x(-45f32.to_radians());
}
const ARM: Vec3 = Vec3::new(0., CAMERA_HEIGHT, CAMERA_HEIGHT);

impl CameraPlugin {
	/// Returns the default camera
	pub fn default() -> impl Bundle {
		(
			Camera3dBundle {
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
				.with(Arm::new(ARM))
				// .with(
				// 	LookAt::new(Vec3::ZERO)
				// 		.tracking_predictive(false)
				// 		.tracking_smoothness(0.),
				// )
				.with(RotationArm::<1>::new(*INITIAL_ROT))
				.with(RotationAccumulator::new(Quat::IDENTITY))
				// .with(Smooth::new_position(0.75).predictive(true))
				.build(),
			RaycastPickCamera::default(),
			MainCamera,
			orbit::PanOrbitCamera {
				radius: ARM.length(),
				..default()
			},
		)
	}
}

#[derive(Debug, Constructor)]
struct RotationArm<const N: usize> {
	pub offset: Quat,
}

impl<const N: usize> RigDriver for RotationArm<N> {
	fn update(&mut self, params: bevy_dolly::dolly::rig::RigUpdateParams) -> Transform {
		Transform {
			rotation: params.parent.rotation.mul_quat(self.offset),
			translation: params.parent.translation,
			scale: Vec3::ONE,
		}
	}
}

#[derive(Debug, Constructor)]
struct RotationAccumulator {
	rot: Quat,
}
impl RigDriver for RotationAccumulator {
	fn update(&mut self, params: bevy_dolly::dolly::rig::RigUpdateParams) -> Transform {
		Transform {
			rotation: params.parent.rotation.mul_quat(self.rot),
			translation: params.parent.translation,
			scale: Vec3::ONE,
		}
	}
}
impl RotationAccumulator {
	fn add_rot(&mut self, rot: Quat) {
		self.rot = self.rot.mul_quat(rot);
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
