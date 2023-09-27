//! Handle main camera

use bevy::{core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseMotion, prelude::*};
use bevy_dolly::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;

use super::player::MainPlayer;
use crate::utils::*;

mod dolly_rig;
mod orbit;
use dolly_rig::*;

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
				..default()
			},
			Rig::builder()
				.with(Position::new(Vec3::ZERO))
				.with(Rotation::new(*INITIAL_ROT))
				.with(OrbitArm::new_from_arm(ARM))
				// .with(
				// 	LookAt::new(Vec3::ZERO)
				// 		.tracking_predictive(false)
				// 		.tracking_smoothness(0.),
				// )
				.with(RotationArm::<1>::new(*INITIAL_ROT))
				// .with(RotationAccumulator::new(Quat::IDENTITY))
				// .with(Smooth::new_position(0.75).predictive(true))
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

	mut scroll: EventReader<MouseMotion>,
	mouse: Res<Input<MouseButton>>,
) {
	let player = player.single();
	let mut rig = camera.single_mut();

	rig.driver_mut::<Position>().position = player.translation;
	rig.driver_mut::<Rotation>().rotation = player.rotation;
	// rig.driver_mut::<LookAt>().target = player.translation;

	let mut scroll_x = 0.;
	let mut scroll_y = 0.;
	if mouse.pressed(MouseButton::Right) {
		for ev in scroll.iter() {
			scroll_x += ev.delta.x / -100.;
			scroll_y += ev.delta.y;
		}
	}

	rig
		.driver_mut::<OrbitArm>()
		.orbit(player.up(), scroll_x, scroll_y);

	scroll.clear();
}
