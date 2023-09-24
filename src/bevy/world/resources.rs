use bevy::prelude::*;

use crate::bevy::utils::MMA;
use rand::Rng;
use static_assertions::*;

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

fn get_random_natural_pixel() -> Box<dyn IsPixel> {
	let mut rng = rand::thread_rng();
	let sum: u32 = RCopper::FREQUENCY as u32 + RDirt::FREQUENCY as u32;
	let num: u32 = rng.gen::<u32>() % sum;
	if num < RCopper::FREQUENCY as u32 {
		Box::new(RCopper::new_random())
	} else {
		Box::new(RDirt::new_random())
	}
}

fn generate_random_resource((meshs, mats, _): &mut MMA) -> PbrBundle {
	let mut rng = rand::thread_rng();

	// let colour = Color::rgb(rng.gen(), rng.gen(), rng.gen());
	let colour = get_random_natural_pixel().get_primary_colour();

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

/// Macro for implementing `IsResource` for a type
macro_rules! impl_pixel_type {
	($type:ty {
		col = $col:expr,
	}) => {
		impl IsPixel for $type {
			fn get_primary_colour(&self) -> Color {
				$col
			}
		}
	};

	($type:ty {
		col = $col:expr,
		res = {},
		// nat = { freq = $freq:expr }
	}) => {
		impl_pixel_type!($type { col = $col, });

		impl IsResource for $type {}

		// impl NaturallyOccurring for $type {
		// 	const FREQUENCY: f32 = $freq;

		// 	fn new_random() -> Self {
		// 		$type {}
		// 	}
		// }
	};
}

pub struct RCopper;
impl_pixel_type!(
	RCopper { col = Color::GREEN, res = {},
	// nat = { freq = 0.3 }
	}
);
impl NaturallyOccurring for RCopper {
	const FREQUENCY: u8 = 30;

	fn new_random() -> Self {
		RCopper {}
	}
}

struct RDirt;
impl_pixel_type!(
	RDirt { col = Color::GRAY, }
);
impl NaturallyOccurring for RDirt {
	const FREQUENCY: u8 = 150;

	fn new_random() -> Self {
		RDirt {}
	}
}

pub trait IsPixel {
	fn get_primary_colour(&self) -> Color;
}
assert_obj_safe!(IsPixel);

/// Whether the pixel can be collected and / or used to build things
pub trait IsResource: IsPixel {}

pub trait NaturallyOccurring: IsPixel {
	/// Weighting used to calculate the frequency of this pixel spawning naturally
	const FREQUENCY: u8;

	fn new_random() -> Self;
}
