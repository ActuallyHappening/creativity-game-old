use super::*;

#[derive(Debug, Clone)]
pub enum StructurePart {
	Pixel {
		px: Pixel,
		relative_location: RelativePixelPoint,
	},
	Thruster {
		thrust: Thruster,
		relative_location: RelativePixelPoint,
	},
}

impl<L> From<(Thruster, L)> for StructurePart
where
	L: Into<RelativePixelPoint>,
{
	fn from((thrust, relative_location): (Thruster, L)) -> Self {
		Self::Thruster {
			thrust,
			relative_location: relative_location.into(),
		}
	}
}

impl<P, L> From<(P, L)> for StructurePart
where
	P: Into<Pixel>,
	L: Into<RelativePixelPoint>,
{
	fn from((px, relative_location): (P, L)) -> Self {
		Self::Pixel {
			px: px.into(),
			relative_location: relative_location.into(),
		}
	}
}
