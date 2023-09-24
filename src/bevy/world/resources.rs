use bevy::prelude::*;

use crate::bevy::utils::MMA;
use rand::Rng;

pub struct WorldResourcesPlugin;
impl Plugin for WorldResourcesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialize_world);
	}
}

fn initialize_world(mut commands: Commands, mut mma: MMA) {
	info!("Initializing world");
	for _ in 0..100 {
		commands.spawn(generate_random_resource(&mut mma));
	}
}

fn generate_random_resource((meshs, mats, _): &mut MMA) -> PbrBundle {
	let mut rng = rand::thread_rng();
	let colour = Color::rgb(rng.gen(), rng.gen(), rng.gen());
	let size = rng.gen_range(1. ..10.);
	let x = rng.gen_range(-100. ..100.);
	let z = rng.gen_range(-100. ..100.);
	PbrBundle {
		material: mats.add(colour.into()),
		mesh: meshs.add(Mesh::from(shape::Cube { size })),
		transform: Transform::from_xyz(x, 0., z),
		..Default::default()
	}
}

pub struct RCopper;
struct RDirt;

pub trait IsPixel {
	const COLOUR: Color;
}

/// Whether the pixel can be collected and / or used to build things
pub trait IsResource: IsPixel {}

pub trait NaturallyOccurring: IsPixel {
	const FREQUENCY: f32;
}

/// Macro for implementing `IsResource` for a list of types
macro_rules! impl_pixel_type {
	($type:ty {
		col = $col:expr,
	}) => {
		impl IsPixel for $type {
			const COLOUR: Color = $col;
		}
	};

	($type:ty {
		col = $col:expr,
		res = {}
	}) => {
		impl IsPixel for $type {
			const COLOUR: Color = $col;
		}

		impl IsResource for $type {}
	};
}

impl_pixel_type!(
	RCopper { col = Color::GREEN, res = {} }
);

pub struct Resource<R: IsResource> {
	resource: R,
}
