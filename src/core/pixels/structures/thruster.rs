use super::*;

#[derive(Debug, Clone, Component)]
pub struct Thruster {
	pub facing: Direction,
	pub flags: ThrustFlags,
}

impl Thruster {
	pub const fn new(facing: Direction, flags: ThrustFlags) -> Thruster {
		Thruster { facing, flags }
	}
}