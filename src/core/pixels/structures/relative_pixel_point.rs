
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Mul)]
pub struct RelativePixelPoint {
	pub x: i8,
	pub y: i8,
	pub z: i8,
}

impl RelativePixelPoint {
	pub const fn new(x: i8, y: i8, z: i8) -> RelativePixelPoint {
		RelativePixelPoint { x, y, z }
	}

	pub fn into_world_vector(self) -> Vec3 {
		Vec3::from(self) * PIXEL_SIZE
	}
}

impl From<RelativePixelPoint> for Vec3 {
	fn from(value: RelativePixelPoint) -> Self {
		Vec3::new(value.x as f32, value.y as f32, value.z as f32)
	}
}

impl From<(i8, i8, i8)> for RelativePixelPoint {
	fn from((x, y, z): (i8, i8, i8)) -> Self {
		Self::new(x, y, z)
	}
}

impl Reflection for RelativePixelPoint {
	fn reflect_horizontally(self) -> Self {
		Self::new(-self.x, self.y, self.z)
	}

	fn reflect_vertically(self) -> Self {
		Self::new(self.x, -self.y, self.z)
	}
}