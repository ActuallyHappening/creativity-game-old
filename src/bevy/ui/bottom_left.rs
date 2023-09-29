use super::*;
use crate::{bevy::player::{MainPlayer, Thrust, RelativeVelocityMagnitudes}, utils::*};

#[derive(Component, Default)]
pub struct Relative<T>(PhantomData<T>);

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
			parent.spawn(
				MaterialMesh2dBundle {
					mesh: mesh.into(),
					transform: Transform::from_rotation(Quat::from_rotation_z(-45f32.to_radians()))
						.translate_z(1.),
					material: mma.mats.add(Color::ORANGE.into()),
					..default()
				}
				.insert(Relative::<ThrustForwards>::default()),
			);
		});
}

pub fn update_bottom_left_camera(
	In(data): In<Thrust<RelativeVelocityMagnitudes>>,
	mut needle: Query<&mut Transform, With<Relative<ThrustForwards>>>,
) {
	let mut needle = needle.single_mut();
	let rot = (-45. + data.forward.clamp(-1.1, 1.1) * 40.).to_radians();

	// debug!("Rotating needle to {}", rot);

	needle.rotation = Quat::from_rotation_z(rot);
}
