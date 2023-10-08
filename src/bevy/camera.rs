//! Handle main camera

use bevy::{
	core_pipeline::{clear_color::ClearColorConfig, tonemapping::Tonemapping},
	input::mouse::{MouseMotion, MouseWheel},
	prelude::*,
};
use bevy_dolly::prelude::*;
use bevy_mod_picking::prelude::RaycastPickCamera;

use super::player::MainPlayer;
use crate::utils::*;

mod dolly_rig;
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
				.with(RotationAccumulator::new(Quat::IDENTITY))
				.with(OrbitArm::new_from_arm(ARM))
				// .with(
				// 	LookAt::new(Vec3::ZERO)
				// 		.tracking_predictive(false)
				// 		.tracking_smoothness(0.),
				// )
				.with(RotationArm::<1>::new(*INITIAL_ROT))
				// .with(Smooth::new_position(0.75).predictive(true))
				.build(),
			// RenderLayers::all(),
			MainCamera,
		)
			.named("Main Camera")
	}
}

/// Added to world in `PlayerPlugin` after player movement system
pub fn handle_camera_movement(
	player: Query<&Transform, (With<MainPlayer>, Without<MainCamera>)>,
	mut camera: Query<&mut Rig, (With<MainCamera>, Without<MainPlayer>)>,

	mut mouse_movements: EventReader<MouseMotion>,
	mouse_clicks: Res<Input<MouseButton>>,
	mut scroll: EventReader<MouseWheel>,
) {
	let player = player.single();
	let mut rig = camera.single_mut();

	let mut orbit_x = 0.;
	let mut orbit_y = 0.;

	for ev in mouse_movements.iter() {
		orbit_x += ev.delta.x / -100.;
		orbit_y += ev.delta.y / 100.;
	}

	let should_reset_orbit = !mouse_clicks.pressed(MouseButton::Right);
	let scroll: f32 = scroll.iter().map(|e| e.y).sum();

	// base pos
	rig.driver_mut::<Position>().position = player.translation + Vec3::Y * PIXEL_SIZE;

	if should_reset_orbit {
		// if not right-click orbitting
		rig.driver_mut::<Rotation>().rotation = player.rotation;
	}

	// zoom
	rig
		.driver_mut::<OrbitArm>()
		.adjust_arm_length(scroll / 100.);

	// orbitting
	let orbit_arm = rig.driver_mut::<OrbitArm>();

	if mouse_clicks.pressed(MouseButton::Middle) {
		orbit_arm
			.orbit(player.up(), player.forward(), 0., 0.)
			.permanent_orbit_horizontal(orbit_x)
			.permanent_orbit_vertical(orbit_y);
	} else if should_reset_orbit {
		orbit_arm
			.orbit(player.up(), player.forward(), 0., 0.)
			.reset_percentage(0.1);
	} else {
		orbit_arm
			.orbit(player.up(), player.forward(), orbit_x, orbit_y);
	}

	mouse_movements.clear();
}
