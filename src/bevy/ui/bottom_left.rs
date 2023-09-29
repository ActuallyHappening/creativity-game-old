use super::*;
use crate::{
	bevy::player::{MainPlayer, RelativeStrength, RelativeVelocityMagnitudes, Thrust},
	utils::*,
};

const FULL_CIRCLE_RADIUS: f32 = 25.;
const SMALLER_CIRCLE_RADIUS: f32 = FULL_CIRCLE_RADIUS - 2.;

#[derive(Component, Debug)]
pub struct NeedleVelocity(ThrustTypes);

#[derive(Component, Debug)]
pub struct NeedleForce(ThrustTypes);

pub fn setup_bottom_left_cam(mut commands: Commands, mut mma: MM2) {
	commands
		.spawn(UiCamera::<BottomLeft>::get_offset_bundle())
		.with_children(|parent| {
			for (i, thrust_type) in ThrustTypes::iter().enumerate() {
				init_ah_circle(parent, thrust_type, i, &mut mma);
			}
		});
}

fn init_ah_circle(parent: &mut ChildBuilder, thrust_tye: ThrustTypes, index: usize, mma: &mut MM2) {
	// Circle
	let circle_center = Vec3::new(
		FULL_CIRCLE_RADIUS + (FULL_CIRCLE_RADIUS * 2. + 3.) * index as f32,
		FULL_CIRCLE_RADIUS,
		0.,
	);
	// larger circle
	let mesh = mma.meshs.add(shape::Circle::new(FULL_CIRCLE_RADIUS).into());
	parent.spawn(MaterialMesh2dBundle {
		mesh: mesh.into(),
		material: mma.mats.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
		transform: Transform::from_translation(circle_center),
		..default()
	});
	// smaller circle
	let mesh = mma
		.meshs
		.add(shape::Circle::new(SMALLER_CIRCLE_RADIUS).into());
	parent.spawn(MaterialMesh2dBundle {
		mesh: mesh.into(),
		material: mma.mats.add(ColorMaterial::from(Color::BLACK)),
		transform: Transform::from_translation(circle_center),
		..default()
	});

	// Needle
	// velocity
	let mesh = mma
		.meshs
		.add(Triangle::new(FULL_CIRCLE_RADIUS / 10., SMALLER_CIRCLE_RADIUS * 0.9).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center).translate_z(3.),
			material: mma.mats.add(Color::ORANGE.into()),
			..default()
		}
		.insert(NeedleVelocity(thrust_tye)),
	);
	// force
	let mesh = mma
		.meshs
		.add(Triangle::new(FULL_CIRCLE_RADIUS / 10., SMALLER_CIRCLE_RADIUS * 0.75).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center).translate_z(2.),
			material: mma.mats.add(Color::BLUE.into()),
			..default()
		}
		.insert(NeedleForce(thrust_tye)),
	);
}

pub fn update_bottom_left_camera(
	In((velocity, relative_strength)): In<(
		Thrust<RelativeVelocityMagnitudes>,
		Thrust<RelativeStrength>,
	)>,
	mut needle_velocity: Query<(&NeedleVelocity, &mut Transform), Without<NeedleForce>>,
	mut needle_force: Query<(&NeedleForce, &mut Transform), Without<NeedleVelocity>>,
) {
	fn update(transform: &mut Transform, data: f32) {
		let angle = data * PI;
		transform.rotation = Quat::from_rotation_z(angle);
	}
	for (NeedleVelocity(thrust_type), mut transform) in needle_velocity.iter_mut() {
		update(
			&mut transform,
			velocity.get_from_type(*thrust_type).clamp(-1.1, 1.1),
		);
	}
	for (NeedleForce(thrust_type), mut transform) in needle_force.iter_mut() {
		update(
			&mut transform,
			relative_strength
				.get_from_type(*thrust_type)
				.clamp(-1.1, 1.1),
		);
	}
}
