use bevy::prelude::{PbrBundle, PointLight, PointLightBundle};

use crate::utils::*;

pub fn spawn_random_star(commands: &mut Commands, MMA { meshs, mats, .. }: &mut MMA) {
	commands
		.spawn(PointLightBundle {
			point_light: PointLight {
				color: random_color(),
				intensity: 10_000_000.,
				..default()
			},
			transform: Transform::from_translation(random_pos()),
			..default()
		})
		.with_children(|parent| {
			parent.spawn(PbrBundle { 
				material: mats.add(Color::WHITE.into()),
				mesh: meshs.add(shape::Icosphere { radius: 100.0, subdivisions: 4 }.try_into().unwrap()),
				..default() });
		});
}

fn random_pos() -> Vec3 {
	let mut p = Vec3::random();
	// while p.length() < 1000. {
	// 	p = Vec3::random();
	// }
	p
}

fn random_color() -> Color {
	Color::rgb(random(), random(), random())
}

#[extension(trait VecExt)]
impl Vec3 {
	fn random() -> Vec3 {
		let mut rng = rand::thread_rng();
		let max = 200.;
		let range = -max..max;
		Vec3::new(
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
		)
	}
}
