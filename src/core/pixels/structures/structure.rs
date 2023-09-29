use super::*;

#[derive(Debug, Clone, Default)]
pub struct Structure {
	parts: Vec<StructurePart>,
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

impl Reflection for Structure {
	fn reflect_horizontally(mut self) -> Self {
		self.parts = self
			.parts
			.into_iter()
			.flat_map(|p| match p {
				StructurePart::Pixel {
					px,
					relative_location,
				} => [
					StructurePart::Pixel {
						px: px.clone(),
						relative_location: relative_location.clone(),
					},
					StructurePart::Pixel {
						px,
						relative_location: relative_location.reflect_horizontally(),
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
						relative_location: relative_location.reflect_horizontally(),
					},
				],
			})
			.collect();
		self
	}

	fn reflect_vertically(mut self) -> Self {
		self.parts = self
			.parts
			.into_iter()
			.flat_map(|p| match p {
				StructurePart::Pixel {
					px,
					relative_location,
				} => [
					StructurePart::Pixel {
						px: px.clone(),
						relative_location: relative_location.clone(),
					},
					StructurePart::Pixel {
						px,
						relative_location: relative_location.reflect_vertically(),
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
						thrust: thrust.reflect_vertically(),
						relative_location: relative_location.reflect_vertically(),
					},
				],
			})
			.collect();
		self
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