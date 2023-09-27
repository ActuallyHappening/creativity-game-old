use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::utils::*;

use super::MainCamera;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component, Debug)]
pub struct PanOrbitCamera {
	/// The "focus point" to orbit around. It is automatically updated when panning the camera
	pub focus: Vec3,
	pub radius: f32,
	pub upside_down: bool,
}

impl Default for PanOrbitCamera {
	fn default() -> Self {
		PanOrbitCamera {
			focus: Vec3::ZERO,
			radius: 5.0,
			upside_down: false,
		}
	}
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn orbit_camera(
	windows: Query<&Window>,
	mut ev_motion: EventReader<MouseMotion>,
	input_mouse: Res<Input<MouseButton>>,

	mut query: Query<(&mut PanOrbitCamera, &mut Rig), With<MainCamera>>,
) {
	if let Ok(window) = windows.get_single() {
		// change input mapping for orbit and panning here
		let orbit_button = MouseButton::Right;

		let mut rotation_move = Vec2::ZERO;
		let mut orbit_button_changed = false;

		if input_mouse.pressed(orbit_button) {
			for ev in ev_motion.iter() {
				rotation_move += ev.delta;
			}
		}
		if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
			orbit_button_changed = true;
		}

		let (mut pan_orbit, mut rig) = query.single_mut();
		let old_rotation = Quat::IDENTITY;
		let mut delta_rotation = Quat::IDENTITY;

		// if orbit_button_changed {
		// 	// only check for upside down when orbiting started or ended this frame
		// 	// if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
		// 	let up = transform.rotation * Vec3::Y;
		// 	pan_orbit.upside_down = up.y <= 0.0;
		// }

		if rotation_move.length_squared() > 0.0 {
			let window = get_primary_window_size(window);
			let delta_x = {
				let delta = rotation_move.x / window.x * std::f32::consts::TAU;
				if pan_orbit.upside_down {
					-delta
				} else {
					delta
				}
			};
			let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
			let yaw = Quat::from_rotation_y(-delta_x);
			let pitch = Quat::from_rotation_x(-delta_y);
			delta_rotation = yaw * old_rotation; // rotate around global y axis
			delta_rotation *= pitch; // rotate around local x axis

			rig.driver_mut::<super::RotationAccumulator>().add_rot(delta_rotation);

			// if any {
			// 	// emulating parent/child to make the yaw/y-axis rotation behave like a turntable
			// 	// parent = x and y rotation
			// 	// child = z-offset
			// 	let rot_matrix = Mat3::from_quat(transform.rotation);
			// 	transform.translation =
			// 		pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
			// }
		}

		// consume any remaining events, so they don't pile up if we don't need them
		// (and also to avoid Bevy warning us about not checking events every frame update)
		ev_motion.clear();
	} else {
		tracing::warn!("Cannot orbit camera if no windows exist");
	}
}

fn get_primary_window_size(window: &Window) -> Vec2 {
	Vec2::new(window.width(), window.height())
}
