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

#[derive(Debug, Clone)]
pub struct Thruster {
	facing: Direction,
}

#[derive(Debug, Clone)]
pub enum Direction {
	Forward,
	Backward,
	Left,
	Right,
	Up,
	Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Mul)]
pub struct RelativePixelPoint {
	x: i8,
	y: i8,
	z: i8,
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

impl Structure {
	pub fn new(parts: impl IntoIterator<Item = impl Into<StructurePart>>) -> Structure {
		Self {
			parts: parts.into_iter().map(|p| p.into()).collect(),
			..default()
		}
	}

	pub fn get_bevy_bundles(
		&self,
		MMA { meshs, mats, .. }: &mut MMA,
	) -> Vec<PbrBundle> {
		self.parts.clone().into_iter().map(|p| match p {
			StructurePart::Thruster(dir) => unimplemented!(),
			StructurePart::Pixel {
				px,
				relative_location,
			} => PbrBundle {
				material: mats.add(px.clone().into()),
				transform: Transform::from_translation(relative_location.into_world_vector()),
				mesh: meshs
						.add(shape::Box::new(PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE).into()),
				..default()
			},
		}).collect()
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

impl From<Pixel> for StandardMaterial {
	fn from(px: Pixel) -> Self {
		px.colour.into()
	}
}
