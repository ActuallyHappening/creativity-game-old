use super::*;

#[derive(Debug, Clone, Component, Constructor)]
pub struct Weapon {
	pub facing: Direction,
}

impl Reflection for Weapon {
	fn reflect_horizontally(self) -> Self {
		Self {
			facing: self.facing.reflect_horizontally(),
		}
	}

	fn reflect_vertically(self) -> Self {
		Self {
			facing: self.facing.reflect_vertically(),
		}
	}
}
