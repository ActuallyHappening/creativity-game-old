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
}

impl ThrustTypes {
	pub fn iter() -> impl Iterator<Item = Self> {
		<Self as IntoEnumIterator>::iter()
	}
}