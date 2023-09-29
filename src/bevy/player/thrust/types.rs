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
pub enum ThrustTypes {
	Forward,
	Right,
	Up,

	TurnLeft,
	TiltUp,
	RollLeft,
}

impl<T: ThrustStage> Thrust<T> {
	pub fn get_from_type(&self, thrust_type: ThrustTypes) -> &T::DimensionType {
		match thrust_type {
			ThrustTypes::Forward => &self.forward,
			ThrustTypes::Right => &self.right,
			ThrustTypes::Up => &self.up,

			ThrustTypes::TurnLeft => &self.turn_left,
			ThrustTypes::TiltUp => &self.tilt_up,
			ThrustTypes::RollLeft => &self.roll_left,
		}
	}

	pub fn set_from_type(&mut self, thrust_type: ThrustTypes, value: T::DimensionType) {
		match thrust_type {
			ThrustTypes::Forward => self.forward = value,
			ThrustTypes::Right => self.right = value,
			ThrustTypes::Up => self.up = value,

			ThrustTypes::TurnLeft => self.turn_left = value,
			ThrustTypes::TiltUp => self.tilt_up = value,
			ThrustTypes::RollLeft => self.roll_left = value,
		}
	}
}

impl ThrustTypes {
	pub fn iter() -> impl Iterator<Item = Self> {
		<Self as IntoEnumIterator>::iter()
	}
}