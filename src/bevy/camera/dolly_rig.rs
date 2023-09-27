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