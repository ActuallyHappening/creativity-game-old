use super::Pixel;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Structure {
	parts: Vec<StructurePart>,
	collider_size: f32,
}

#[derive(Debug, Clone)]
pub enum StructurePart {
	Pixel {
		px: Pixel,
		relative_location: RelativePixelPoint,
	},
	Thruster(Thruster),
}

#[derive(Debug, Clone,)]
pub struct Thruster {
	facing: Direction,
}

#[derive(Debug, Clone,)]
pub enum Direction {
	Forward,
	Backward,
	Left,
	Right,
	Up,
	Down
}

#[derive(Debug, Clone, PartialEq, Eq, Default,)]
pub struct RelativePixelPoint {
	x: i8,
	y: i8,
	z: i8,
}

impl RelativePixelPoint {
	pub const fn new(x: i8, y: i8, z: i8) -> RelativePixelPoint {
		RelativePixelPoint { x, y, z }
	}
}

impl From<(i8, i8, i8)> for RelativePixelPoint {
	fn from((x, y, z): (i8, i8, i8)) -> Self {
		Self::new(x, y, z)
	}
}

impl Thruster {
	pub const fn new(facing: Direction) -> Thruster {
		Thruster { facing }
	}
}

impl From<Thruster> for StructurePart {
	fn from(thruster: Thruster) -> Self {
		Self::Thruster(thruster)
	}
}

impl<T> From<(Pixel, T)> for StructurePart where T: Into<RelativePixelPoint> {
	fn from((px, relative_location): (Pixel, T)) -> Self {
		Self::Pixel { px, relative_location: relative_location.into() }
	}
}

impl Structure {
	pub fn new(parts: impl IntoIterator<Item = impl Into<StructurePart>>) -> Structure {
		Self {
			parts: parts.into_iter().map(|p| p.into()).collect(),
			..default()
		}
	}
}

impl Default for Structure {
	fn default() -> Self {
		Structure {
			parts: vec![],
			collider_size: 10.,
		}
	}
}
