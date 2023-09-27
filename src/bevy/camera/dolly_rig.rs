use bevy_dolly::dolly::rig::RigUpdateParams;

use crate::utils::*;

#[derive(Debug, Constructor)]
pub struct RotationArm<const N: usize> {
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
pub struct RotationAccumulator {
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
	pub fn add_rot(&mut self, rot: Quat) {
		self.rot = self.rot.mul_quat(rot);
	}
}

#[derive(Debug, Constructor)]
pub struct OrbitArm {
	offset: Vec3,

	local_up: Option<Vec3>,

	total_sideways_rot: f32,
	// rot_vertically: f32,
}

impl OrbitArm {
	fn get_radius(&self) -> f32 {
		self.offset.length()
	}

	pub fn new_from_arm(offset: Vec3) -> Self {
		Self {
			offset,
			local_up: None,
			total_sideways_rot: 0.,
			// rot_vertically: 0.,
		}
	}

	fn orbit_horizontal(&mut self, angle: f32) {
		self.total_sideways_rot += angle;
	}

	fn orbit_vertical(&mut self, angle: f32) {
		// self.rot_vertically += angle;
	}

	pub fn orbit(&mut self, local_up: Vec3, horizontal_rot: f32, vertical_rot: f32) {
		self.local_up = Some(local_up);

		self.orbit_horizontal(horizontal_rot);
		self.orbit_vertical(vertical_rot);
	}
}

impl RigDriver for OrbitArm {
	fn update(&mut self, mut params: bevy_dolly::dolly::rig::RigUpdateParams) -> Transform {
		match self.local_up {
			None => {
				tracing::error!("You must call `.orbit()` on [OrbitalArm] before using it every frame!");
				params.parent.with_translation(self.offset)
			}
			Some(local_up) => {
				let mut transform = *params.parent;

				// let rot = Quat::from_rotation_y();
				transform.rotate_axis(local_up, self.total_sideways_rot);

				transform.translation += transform.rotation * self.offset; // arm
				self.local_up = None;
				transform
			}
		}
	}
}
