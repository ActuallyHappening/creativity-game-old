use bevy::prelude::PbrBundle;

use crate::utils::*;

pub fn spawn_random_star(commands: &mut Commands, MMA { meshs, mats, .. }: &mut MMA) {
	// commands
	// 	.spawn(PointLightBundle {
	// 		point_light: PointLight {
	// 			color: random_color(),
	// 			intensity: 1e20,
	// 			range: 1e20,
	// 			// radius: ,
	// 			..default()
	// 		},
	// 		transform: Transform::from_translation(random_pos()),
	// 		..default()
	// 	})
	// 	.with_children(|commands| {
	commands.spawn(PbrBundle {
		material: mats.add(StandardMaterial {
			base_color: Color::WHITE,
			emissive: Color::WHITE,
			unlit: true,
			..default()
		}),
		mesh: meshs.add(
			shape::Icosphere {
				radius: 150.0,
				subdivisions: 4,
			}
			.try_into()
			.unwrap(),
		),
		transform: Transform::from_translation(random_pos()),
		..default()
	});
	// });
}

fn random_pos() -> Vec3 {
	let mut p = Vec3::random();
	while p.length() < 10_000. {
		p = Vec3::random();
	}
	p
}

// fn random_color() -> Color {
// 	Color::rgb(random(), random(), random())
// }

#[extension(trait VecExt)]
impl Vec3 {
	fn random() -> Vec3 {
		let mut rng = rand::thread_rng();
		let max = 200_000.;
		let range = -max..max;
		Vec3::new(
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
			rng.gen_range(range.clone()),
		)
	}
}
