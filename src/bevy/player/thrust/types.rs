use crate::utils::*;

use super::*;

// macro_rules! thrust_type {
// 	($(pub struct $name:ident;)*) => {
// 		$(
// 			#[derive(Component, Debug, Default)]
// 			pub struct $name;
// 		)*
// 	};
// }

// thrust_type!(
// 	pub struct ThrustForwards;
// 	pub struct ThrustRight;
// 	pub struct ThrustUpwards;
// );

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum ThrustType {
	Forward,
	Right,
	Up,

	TurnLeft,
	TiltUp,
	RollLeft,
}

impl<T: ThrustStage> Thrust<T> {
	pub fn get_from_type(&self, thrust_type: ThrustType) -> &T::DimensionType {
		match thrust_type {
			ThrustType::Forward => &self.forward,
			ThrustType::Right => &self.right,
			ThrustType::Up => &self.up,

			ThrustType::TurnLeft => &self.turn_left,
			ThrustType::TiltUp => &self.tilt_up,
			ThrustType::RollLeft => &self.roll_left,
		}
	}

	pub fn set_from_type(&mut self, thrust_type: ThrustType, value: T::DimensionType) {
		match thrust_type {
			ThrustType::Forward => self.forward = value,
			ThrustType::Right => self.right = value,
			ThrustType::Up => self.up = value,

			ThrustType::TurnLeft => self.turn_left = value,
			ThrustType::TiltUp => self.tilt_up = value,
			ThrustType::RollLeft => self.roll_left = value,
		}
	}
}

impl ThrustType {
	pub fn iter() -> impl Iterator<Item = Self> {
		<Self as IntoEnumIterator>::iter()
	}

	pub const fn ah_circle_name(&self) -> &'static str {
		match self {
			Self::Forward => "forward",
			Self::Right => "right",
			Self::Up => "up",

			Self::TurnLeft => "turn",
			Self::TiltUp => "tilt",
			Self::RollLeft => "roll",
		}
	}
}
