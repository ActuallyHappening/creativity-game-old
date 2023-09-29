use super::Pixel;
use crate::utils::*;

mod structure;
pub use structure::Structure;

mod structure_part;
use structure_part::StructurePart;

pub trait Reflection {
	fn reflect_horizontally(self) -> Self;
	fn reflect_vertically(self) -> Self;
}

mod thruster;
pub use thruster::Thruster;

mod thruster_flags;
pub use thruster_flags::ThrustFlags;

mod direction;
pub use direction::Direction;

mod relative_pixel_point;
use relative_pixel_point::RelativePixelPoint;


impl From<Pixel> for StandardMaterial {
	fn from(px: Pixel) -> Self {
		px.colour.into()
	}
}
