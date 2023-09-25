use bevy::prelude::*;
use bevy_mod_picking::prelude::{On, Pointer};
use bevy_mod_picking::events::Click;
use tracing::info;

use self::traits::{IsPixel, NaturallyOccurring, APixel};
use crate::bevy::{utils::*, world::resources::world_gen::WorldGen};

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

	let world = WorldGen::<WORLD_WIDTH>::new(4);
	for (x, height, z) in world {
		for y in 0..height {
			let point = WorldPoint { x, y, z };
			let bundle = generate_natural_pixel(point, &mut mma);
			commands.spawn(bundle);
		}
	}
}

mod world_gen;

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
	unreachable!("Failed to pick random natural pixel");
}

fn generate_natural_pixel(point: WorldPoint, (meshs, mats, _): &mut MMA) -> impl Bundle {
	let material = pick_random_natural_pixel().get_pixel().material;

	(PbrBundle {
		material: mats.add(material),
		mesh: meshs.add(Mesh::from(shape::Cube { size: PIXEL_SIZE })),
		transform: Transform::from_translation(point.into_bevy_vector()),
		..Default::default()
	}, PixelComponent {
		point,
		// pixel: material,
	},
	On::<Pointer<Click>>::run(|| {
		info!("Clicked!");
	})
)
}

#[derive(Component)]
pub struct PixelComponent {
	pub point: WorldPoint,
	// pub pixel: APixel,
}