use super::*;
use crate::{
	bevy::player::{BrakingInfo, MainPlayer, RelativeStrength, RelativeVelocityMagnitudes, Thrust},
	utils::*,
};

const FULL_CIRCLE_RADIUS: f32 = 25.;
const SMALLER_CIRCLE_RADIUS: f32 = FULL_CIRCLE_RADIUS - 2.;
const TINY_CIRCLE_RADIUS: f32 = 4.;

const DISABLED_INPUT_COL: Color = Color::GRAY;
const USER_ENABLED_COL: Color = Color::GREEN;
const BRAKING_ENABLED_COL: Color = Color::RED;

#[derive(Component, Debug)]
pub struct NeedleVelocity(ThrustType);

#[derive(Component, Debug)]
pub struct NeedleStrength(ThrustType);

#[derive(Component, Debug, Constructor)]
pub struct InputFlag {
	is_right: bool,
	thrust_type: ThrustType,
}

pub fn setup_bottom_left_cam(mut commands: Commands, mut mma: MM2) {
	commands
		.spawn(UiCamera::<BottomLeft>::get_offset_bundle())
		.with_children(|parent| {
			for (i, thrust_type) in ThrustType::iter().enumerate() {
				init_ah_circle(parent, thrust_type, i, &mut mma);
			}
		});
}

fn init_ah_circle(parent: &mut ChildBuilder, thrust_tye: ThrustType, index: usize, mma: &mut MM2) {
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
		.insert(NeedleStrength(thrust_tye)),
	);

	// Input flags
	let mesh = mma.meshs.add(shape::Circle::new(TINY_CIRCLE_RADIUS).into());
	// left
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.clone().into(),
			transform: Transform::from_translation(circle_center)
				.translate_z(1.)
				.translate_x(-TINY_CIRCLE_RADIUS),
			material: mma.mats.add(DISABLED_INPUT_COL.into()),
			..default()
		}
		.insert(InputFlag::new(false, thrust_tye)),
	);
	// right
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center)
				.translate_z(1.)
				.translate_x(TINY_CIRCLE_RADIUS),
			material: mma.mats.add(DISABLED_INPUT_COL.into()),
			..default()
		}
		.insert(InputFlag::new(true, thrust_tye)),
	);
}

pub fn update_bottom_left_camera(
	In((velocity, relative_strength, BrakingInfo(is_braking, flags))): In<(
		Thrust<RelativeVelocityMagnitudes>,
		Thrust<RelativeStrength>,
		BrakingInfo,
	)>,
	mut needle_velocity: Query<(&NeedleVelocity, &mut Transform), Without<NeedleStrength>>,
	mut needle_force: Query<(&NeedleStrength, &mut Transform), Without<NeedleVelocity>>,
	mut input_flags: Query<(&InputFlag, &mut Handle<ColorMaterial>)>,

	mut mma: MM2,
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

	for (NeedleStrength(thrust_type), mut transform) in needle_force.iter_mut() {
		update(
			&mut transform,
			relative_strength
				.get_from_type(*thrust_type)
				.clamp(-0.9, 0.9),
		);
	}

	for (
		InputFlag {
			is_right,
			thrust_type,
		},
		material,
	) in input_flags.iter_mut()
	{
		let flag = flags.get_from_type(*thrust_type);
		match flag.as_ref() {
			None => {
				*material.into_inner() = mma.mats.add(DISABLED_INPUT_COL.into());
			}
			Some(flag_right) => {
				if *is_right == *flag_right {
					let handle = mma.mats.add(
						if is_braking {
							BRAKING_ENABLED_COL
						} else {
							USER_ENABLED_COL
						}
						.into(),
					);

					*material.into_inner() = handle;
				} else {
					*material.into_inner() = mma.mats.add(DISABLED_INPUT_COL.into());
				}
			}
		}
	}
}
