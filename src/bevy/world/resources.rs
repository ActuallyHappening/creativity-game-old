use bevy::prelude::*;

use self::traits::{IsPixel, NaturallyOccurring};
use crate::bevy::{utils::*, world::resources::world_gen::WorldGen};

use rand::Rng;

mod traits;

mod copper;
pub use copper::RCopper;
mod dirt;
pub use dirt::RDirt;

pub struct WorldResourcesPlugin;
impl Plugin for WorldResourcesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, initialize_world);
	}
}

fn initialize_world(mut commands: Commands, mut mma: MMA) {
	info!("Initializing world");

	let world = WorldGen::<WORLD_WIDTH>::new(7);
	for (x, y, z) in world {
		let point = WorldPoint { x, y, z };
		let bundle = generate_natural_pixel(point, &mut mma);
		commands.spawn(bundle);
	}
}

mod world_gen {
	use noise::{Fbm, NoiseFn, Perlin};

	pub struct WorldGen<const TW: usize> {
		/// Matrix of numbers between 0 and 1
		heights: [[f64; TW]; TW],
		max_height: usize,
	}

	impl<const TW: usize> WorldGen<TW> {
		pub fn new(max_height: usize) -> Self {
			let noise = Fbm::<Perlin>::new(rand::random());
			let mut ret = Self {
				heights: [[0.; TW]; TW],
				max_height,
			};
			for x in 0..TW {
				for z in 0..TW {
					let h = noise.get([x as f64, z as f64]);
					ret.heights[x][z] = h;
				}
			}

			ret
		}

		fn get_height(&self, x: i32, z: i32) -> f64 {
			assert!(
				x <= TW as i32 / 2 && z <= TW as i32 / 2,
				"x and z must be less than or equal to {}",
				TW / 2
			);

			let x: usize = (x + TW as i32 / 2).try_into().expect("x is too small");
			assert!(
				x < TW,
				"X value of {x} is larger than {} which is half of the total width TM={}",
				TW / 2,
				TW
			);
			let z: usize = (z + TW as i32 / 2).try_into().expect("z is too small");
			assert!(
				z < TW,
				"Z value of {z} is larger than {} which is half of the total width TM={}",
				TW / 2,
				TW
			);

			let ret = (self.heights[x][z] + 1.) / 2.;

			assert!(
				(0. ..=1.).contains(&ret),
				"Height at ({}, {}) is {} which is not expected",
				x,
				z,
				ret
			);

			ret
		}

		fn normalized_into_height(&self, height: f64) -> i32 {
			(height * self.max_height as f64).round() as i32
		}
	}

	/// Iterator yielding (i32, i32, i32) with x&z in range [-TW/2, TW/2) and y in range [0, 1)
	impl<const TW: usize> IntoIterator for WorldGen<TW> {
		type Item = (i32, i32, i32);
		type IntoIter = std::vec::IntoIter<Self::Item>;

		fn into_iter(self) -> Self::IntoIter {
			let radius: i32 = (TW / 2).try_into().expect("TW is too large");
			let mut ret = Vec::with_capacity(TW * TW);
			for x in -radius..radius {
				for z in -radius..radius {
					ret.push((x, self.normalized_into_height(self.get_height(x, z)), z));
				}
			}
			ret.into_iter()
		}
	}

	#[test]
	fn test_world_gen() {
		let world = WorldGen::<10>::new(7);

		for x in -5..5 {
			for z in -5..5 {
				let h = world.get_height(x, z);
				println!("Height at ({}, {}): {}", x, z, h);
			}
		}
	}
}

fn all_natural_pixels_randomized() -> Vec<(Box<dyn IsPixel>, u8)> {
	/// macro to return a tuple of a pixel and its frequency
	macro_rules! natural_pixels {
		($($type:ty),*) => {
			vec![$(
				(Box::new(<$type>::new_random()), <$type>::FREQUENCY),
			)*]
		};
	}
	// [
	// 	(Box::new(RCopper::new_random()), RCopper::FREQUENCY),
	// 	(Box::new(RDirt::new_random()), RDirt::FREQUENCY),
	// ]
	natural_pixels!(RCopper, RDirt)
}
fn pick_random_natural_pixel() -> Box<dyn IsPixel> {
	let pixels = all_natural_pixels_randomized();
	let sum: u32 = pixels
		.iter()
		.map(|(_, freq)| freq)
		.fold(0, |acc, freq| acc + *freq as u32);
	let r = rand::random::<u32>() % sum;

	let mut acc = 0;
	for (pixel, freq) in pixels.into_iter() {
		acc += freq as u32;
		if acc >= r {
			return pixel;
		}
	}
	panic!("Failed to pick random natural pixel");
}

fn generate_natural_pixel(point: WorldPoint, (meshs, mats, _): &mut MMA) -> PbrBundle {
	let colour = pick_random_natural_pixel().get_primary_colour();

	PbrBundle {
		material: mats.add(colour.into()),
		mesh: meshs.add(Mesh::from(shape::Cube { size: PIXEL_SIZE })),
		transform: Transform::from_translation(point.into_bevy_vector()),
		..Default::default()
	}
}
