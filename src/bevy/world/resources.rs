use bevy::prelude::*;

use crate::bevy::utils::MMA;
use rand::Rng;

pub struct WorldResourcesPlugin;
impl Plugin for WorldResourcesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, initialize_world);
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