//! Handle main camera

use bevy::{
	core_pipeline::{clear_color::ClearColorConfig, tonemapping::Tonemapping},
	input::{
		keyboard,
		mouse::{MouseMotion, MouseWheel},
	},
	prelude::*,
};

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
				projection: Projection::Perspective(PerspectiveProjection {
					far: 10_000_000.,
					..default()
				}),

				#[cfg(feature = "hanabi_particles")]
				tonemapping: Tonemapping::None,

				..default()
			},
			MainCamera,
		)
			.named("Main Camera")
	}
}