use crate::{utils::*, bevy::player::MainPlayer};
use super::*;

pub fn setup_bottom_left_cam(mut commands: Commands, mut mma: MM2) {
	commands
		.spawn(UiCamera::<BottomLeft>::get_offset_bundle())
		.with_children(|parent| {
			// Circle
			let mesh = mma.meshs.add(shape::Circle::new(100.).into());
			parent.spawn(MaterialMesh2dBundle {
				mesh: mesh.into(),
				material: mma.mats.add(ColorMaterial::from(Color::BLACK)),
				transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
				..default()
			});

			// Needle
			let mesh = mma.meshs.add(Triangle::new(10., 90.).into());
			parent.spawn(MaterialMesh2dBundle {
				mesh: mesh.into(),
				transform: Transform::from_rotation(Quat::from_rotation_z(-45f32 .to_radians())).translate_z(1.),
				material: mma.mats.add(Color::ORANGE.into()),
				..default()
			}.insert(ThrustForwards));
		});
}

pub fn update_bottom_left_camera(player: Query<&MainPlayer>, mut needle: Query<&mut Transform, With<ThrustForwards>>) {
	let player = player.single();
	let mut needle = needle.single_mut();
	let rot = (-45. + player.relative_thrust.forward * 40.).to_radians();

	debug!("Rotating needle to {}", rot);

	needle.rotation = Quat::from_rotation_z(rot);
}