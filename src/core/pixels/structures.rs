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
	Thruster {
		thrust: Thruster,
		relative_location: RelativePixelPoint,
	},
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

impl Direction {
	/// From/Into impl, but use explicit method where possible
	pub fn into_rotation(self) -> Quat {
		impl From<Direction> for Quat {
			fn from(value: Direction) -> Self {
				value.into_rotation()
			}
		}

		match self {
			Self::Backward => Quat::from_rotation_x(90f32.to_radians()),
			Self::Forward => Quat::from_rotation_x(-90f32.to_radians()),
			Self::Left => Quat::from_rotation_z(90f32.to_radians()),
			Self::Right => Quat::from_rotation_z(-90f32.to_radians()),
			Self::Up => Quat::IDENTITY,
			Self::Down => Quat::from_rotation_z(180f32.to_radians()),
		}
	}
}

impl Thruster {
	pub const fn new(facing: Direction) -> Thruster {
		Thruster { facing }
	}
}

impl<L> From<(Thruster, L)> for StructurePart
where
	L: Into<RelativePixelPoint>,
{
	fn from((thrust, relative_location): (Thruster, L)) -> Self {
		Self::Thruster {
			thrust: thrust.into(),
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

pub enum StructureBundle {
	Pixel(PbrBundle),
	Thruster(PbrBundle, ParticleEffectBundle),
}

impl Structure {
	pub fn new(parts: impl IntoIterator<Item = impl Into<StructurePart>>) -> Self {
		Self {
			parts: parts.into_iter().map(|p| p.into()).collect(),
			..default()
		}
	}

	pub fn with(mut self, part: impl IntoIterator<Item = impl Into<StructurePart>>) -> Self {
		self.parts.extend(part.into_iter().map(|p| p.into()));
		self
	}

	pub fn spawn_bevy_bundles(
		&self,
		mma: &mut MMA,
		effects: ResMut<Assets<EffectAsset>>,
	) -> Vec<StructureBundle> {
		let effects = effects.into_inner();
		self
			.parts
			.clone()
			.into_iter()
			.map(move |p| match p {
				StructurePart::Thruster {
					thrust,
					relative_location,
				} => StructureBundle::Thruster(
					PbrBundle {
						material: mma.mats.add(Color::ORANGE_RED.into()),
						transform: Transform::from_translation(relative_location.into_world_vector()),
						mesh: mma.meshs.add(shape::Cube::new(PIXEL_SIZE / 2.).into()),
						..default()
					},
					{
						let mut particles = gen_particles(effects);
						particles.transform = Transform::from_rotation(thrust.facing.into_rotation());
						particles
					},
				),
				StructurePart::Pixel {
					px,
					relative_location,
				} => StructureBundle::Pixel(PbrBundle {
					material: mma.mats.add(px.clone().into()),
					transform: Transform::from_translation(relative_location.into_world_vector()),
					mesh: mma.meshs.add(shape::Cube::new(PIXEL_SIZE).into()),
					..default()
				}),
			})
			.collect()
	}
}

impl StructureBundle {
	pub fn spawn_to_parent(self, parent: &mut ChildBuilder) {
		match self {
			StructureBundle::Pixel(bundle) => {
				parent.spawn(bundle);
			}
			StructureBundle::Thruster(bundle, effect) => {
				parent.spawn(bundle).with_children(|parent| {
					parent.spawn(effect);
				});
			}
		};
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
