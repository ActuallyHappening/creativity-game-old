use bevy::prelude::*;

use self::traits::{IsPixel, NaturallyOccurring};
use crate::bevy::utils::*;

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
	for x in -100..100 {
		for y in -100..100 {
			commands.spawn(generate_natural_pixel((x, y), &mut mma));
		}
	}
}

fn all_natural_pixels_randomized() -> [(Box<dyn IsPixel>, u8); 2] {
	[
		(Box::new(RCopper::new_random()), RCopper::FREQUENCY),
		(Box::new(RDirt::new_random()), RDirt::FREQUENCY),
	]
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

fn generate_natural_pixel((x, y): (i8, i8), (meshs, mats, _): &mut MMA) -> PbrBundle {
	let colour = pick_random_natural_pixel().get_primary_colour();

	let size = PIXEL_SIZE;
	let x = x as f32 * PIXEL_SIZE;
	let z = y as f32 * PIXEL_SIZE;

	PbrBundle {
		material: mats.add(colour.into()),
		mesh: meshs.add(Mesh::from(shape::Cube { size })),
		transform: Transform::from_xyz(x, 0., z),
		..Default::default()
	}
}
