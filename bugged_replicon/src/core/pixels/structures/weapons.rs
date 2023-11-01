/// Not used directly as a Component, see [Weapon]
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
pub struct WeaponFlags {
	/// edited from systems
	pub(self) try_fire_this_frame: Option<bool>,
}

use super::*;

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Weapon {
	pub facing: Direction,
	pub flags: WeaponFlags,
}

impl Weapon {
	pub fn new(facing: Direction) -> Self {
		Self {
			facing,
			flags: WeaponFlags::default(),
		}
	}
}

impl Reflection for Weapon {
	fn reflect_horizontally(mut self) -> Self {
		self.facing = self.facing.reflect_horizontally();
		self
	}

	fn reflect_vertically(mut self) -> Self {
		self.facing = self.facing.reflect_vertically();
		self
	}
}
