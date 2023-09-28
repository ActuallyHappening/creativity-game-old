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

trait Reflection {
	fn reflect_horizontally(self) -> Self;
	fn reflect_vertically(self) -> Self;
}

mod thruster;
use thruster::*;

mod thruster_flags;
use thruster_flags::*;

mod direction;
pub use direction::Direction;

mod relative_pixel_point;
use relative_pixel_point::*;

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

pub enum StructureBundle {
	Pixel(PbrBundle),
	Thruster {
		visual: PbrBundle,
		data: Thruster,
		particles: ParticleEffectBundle,
	},
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

	pub fn reflect_horizontally(mut self) -> Self {
		self.parts = self
			.parts
			.into_iter()
			.map(|p| match p {
				StructurePart::Pixel {
					px,
					relative_location,
				} => [
					StructurePart::Pixel {
						px: px.clone(),
						relative_location: relative_location.clone(),
					},
					StructurePart::Pixel {
						px: px.clone(),
						relative_location: RelativePixelPoint::new(
							-relative_location.x,
							relative_location.y,
							relative_location.z,
						),
					},
				],
				StructurePart::Thruster {
					thrust,
					relative_location,
				} => [
					StructurePart::Thruster {
						thrust: thrust.clone(),
						relative_location: relative_location.clone(),
					},
					StructurePart::Thruster {
						thrust: thrust.reflect_horizontally(),
						relative_location: RelativePixelPoint::new(
							-relative_location.x,
							relative_location.y,
							relative_location.z,
						),
					},
				],
			})
			.flatten()
			.collect();
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
				} => StructureBundle::Thruster {
					visual: PbrBundle {
						material: mma.mats.add(Color::ORANGE_RED.into()),
						transform: Transform::from_translation(
							relative_location.into_world_vector()
								- (PIXEL_SIZE / 3.) * thrust.facing.into_direction_vector(),
						),
						mesh: mma.meshs.add(shape::Cube::new(PIXEL_SIZE / 2.).into()),
						..default()
					},
					particles: {
						let mut particles = gen_particles(effects);
						particles.transform = Transform::from_rotation(thrust.facing.into_rotation());
						particles
					},
					data: thrust,
				},
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
			StructureBundle::Thruster {
				visual,
				particles,
				data,
			} => {
				parent.spawn(visual).with_children(|parent| {
					parent.spawn(particles.insert(data));
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
