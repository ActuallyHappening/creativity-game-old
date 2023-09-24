use bevy::prelude::*;

use self::traits::{IsPixel, NaturallyOccurring};
use crate::bevy::utils::*;
use noise::{NoiseFn, Perlin, Seedable};
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
	let mut rng = rand::thread_rng();
	
	let r = rng.gen();
	let perlin_noise = Perlin::new(r);

	for x in -(WORLD_WIDTH as i32)..WORLD_WIDTH as i32 {
		for z in -(WORLD_WIDTH as i32)..WORLD_WIDTH as i32 {
			let y = perlin_noise.get([x as f64, z as f64]) as i32;
			let point = WorldPoint { x, y, z };
			commands.spawn(generate_natural_pixel(point, &mut mma));
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
		.iter().map(|(_, freq) | freq).fold(0, |acc, freq| acc + *freq as u32);
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
