//! Handle main camera

use bevy::{
	core_pipeline::{clear_color::ClearColorConfig, tonemapping::Tonemapping},
	input::mouse::MouseMotion,
	prelude::*,
};
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

				#[cfg(feature = "hanabi_particles")]
				tonemapping: Tonemapping::None,

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

	let mut scroll_x = 0.;
	let mut scroll_y = 0.;
	let should_reset_orbit = if mouse.pressed(MouseButton::Right) {
		for ev in scroll.iter() {
			scroll_x += ev.delta.x / -100.;
			scroll_y += ev.delta.y / 100.;
		}

		// if scroll_x != 0. || scroll_y != 0. {
		// 	info!("Scroll x: {} y: {}", scroll_x, scroll_y);
		// }

		if scroll_y.abs() < scroll_x.abs() {
			scroll_y = 0.;
		} else if scroll_x.abs() < scroll_y.abs() {
			scroll_x = 0.;
		}

		false
	} else {
		true
	};

	rig.driver_mut::<Position>().position = player.translation;
	if should_reset_orbit {
		rig.driver_mut::<Rotation>().rotation = player.rotation;
	}

	rig
		.driver_mut::<OrbitArm>()
		.orbit(player.up(), player.forward(), scroll_x, scroll_y)
		.reset_percentage(if should_reset_orbit { 0.1 } else { 0. });

	scroll.clear();
}
