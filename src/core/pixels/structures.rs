use super::Pixel;
use crate::utils::*;

mod structure;
pub use structure::Structure;

mod structure_bundle;
pub use structure_bundle::StructureBundle;

mod structure_part;
pub use structure_part::StructurePart;

pub trait Reflection {
	fn reflect_horizontally(self) -> Self;
	fn reflect_vertically(self) -> Self;
}

mod thruster;
pub use thruster::Thruster;

mod thruster_flags;
pub use thruster_flags::ThrusterFlags;

mod weapons;
pub use weapons::*;

mod direction;
pub use direction::Direction;

mod relative_pixel_point;
pub use relative_pixel_point::RelativePixelPoint;

impl From<Pixel> for StandardMaterial {
	fn from(px: Pixel) -> Self {
		px.colour.into()
	}
}
