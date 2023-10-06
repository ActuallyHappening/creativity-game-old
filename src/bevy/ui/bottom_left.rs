use super::*;
use crate::bevy::player::{
	MainPlayer, RelativeStrength, RelativeVelocityMagnitudes, Thrust, ThrustReactions,
	ThrustReactionsStage,
};
use crate::utils::Text2dBundle;

const FULL_CIRCLE_RADIUS: f32 = 25.;
const BORDER_CIRCLE_RADIUS: f32 = FULL_CIRCLE_RADIUS - 3.;
const SMALLER_CIRCLE_RADIUS: f32 = FULL_CIRCLE_RADIUS - 4.;
const TINY_CIRCLE_RADIUS: f32 = 5.;

const DISABLED_INPUT_COL: Color = Color::GRAY;
const USER_ENABLED_COL: Color = Color::GREEN;
const BRAKING_ENABLED_COL: Color = Color::RED;
const ARTIFICIAL_FRICTION_ENABLED_COL: Color = Color::YELLOW;

#[derive(Component, Debug)]
pub struct NeedleVelocity(ThrustType);

#[derive(Component, Debug)]
pub struct NeedleStrength(ThrustType);

#[derive(Component, Debug, Constructor)]
pub struct InputFlag {
	is_right: bool,
	thrust_type: ThrustType,
}

#[derive(Component, Debug, Constructor)]
pub struct BorderCircle(ThrustType);

pub fn setup_bottom_left_cam(mut commands: Commands, mut mma: MM2) {
	for thrust_type in ThrustType::iter() {
		init_ah_circle(&mut commands, thrust_type, &mut mma);
	}
}

impl ThrustType {
	fn get_trust_offset(&self) -> (i8, i8) {
		match self {
			ThrustType::Forward => (0, 0),
			ThrustType::Up => (1, 0),
			ThrustType::Right => (2, 0),

			ThrustType::TiltUp => (0, 1),
			ThrustType::RollLeft => (1, 1),
			ThrustType::TurnLeft => (0, 2),
		}
	}
}

fn init_ah_circle(parent: &mut Commands, thrust_type: ThrustType, mma: &mut MM2) {
	// Circle
	const MARGIN: f32 = 3.;
	let mut layer_counter: f32 = 0.;
	let circle_center = Vec3::new(
		FULL_CIRCLE_RADIUS
			+ (FULL_CIRCLE_RADIUS * 2. + MARGIN) * thrust_type.get_trust_offset().0 as f32,
		FULL_CIRCLE_RADIUS
			+ (FULL_CIRCLE_RADIUS * 2. + MARGIN) * thrust_type.get_trust_offset().1 as f32,
		layer_counter,
	);
	// larger circle
	let mesh = mma.meshs.add(shape::Circle::new(FULL_CIRCLE_RADIUS).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			material: mma.mats.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
			transform: Transform::from_translation(circle_center).translate_z(layer_counter),
			..default()
		}
		.pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;
	// border radius
	let mesh = mma
		.meshs
		.add(shape::Circle::new(BORDER_CIRCLE_RADIUS).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			material: mma.mats.add(ColorMaterial::from(Color::BLACK)),
			transform: Transform::from_translation(circle_center).translate_z(layer_counter),
			..default()
		}
		.insert(BorderCircle(thrust_type))
		.not_pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;
	// smaller circle
	let mesh = mma
		.meshs
		.add(shape::Circle::new(SMALLER_CIRCLE_RADIUS).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			material: mma.mats.add(ColorMaterial::from(Color::BLACK)),
			transform: Transform::from_translation(circle_center).translate_z(layer_counter),
			..default()
		}
		.not_pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;

	// Text
	let text = thrust_type.ah_circle_name();
	parent.spawn(
		crate::utils::Text2dBundle::new(text, Font::Medium, 7., Color::RED, mma)
			.translate(circle_center)
			.translate_z(layer_counter)
			.translate_y(-INNER_RADIUS_OFFSET)
			.not_pickable()
			.render_layer(BottomLeft),
	);
	layer_counter += 1.;

	// Input flags
	let mesh = mma.meshs.add(shape::Circle::new(TINY_CIRCLE_RADIUS).into());
	const INNER_RADIUS_OFFSET: f32 = SMALLER_CIRCLE_RADIUS / 2.;
	// left
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.clone().into(),
			transform: Transform::from_translation(circle_center)
				.translate_z(layer_counter)
				.translate_x(-INNER_RADIUS_OFFSET),
			material: mma.mats.add(DISABLED_INPUT_COL.into()),
			..default()
		}
		.insert(InputFlag::new(false, thrust_type))
		.not_pickable()
		.render_layer(BottomLeft),
	);
	// right
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center)
				.translate_z(layer_counter)
				.translate_x(INNER_RADIUS_OFFSET),
			material: mma.mats.add(DISABLED_INPUT_COL.into()),
			..default()
		}
		.insert(InputFlag::new(true, thrust_type))
		.not_pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;

	// Needle
	// force
	let mesh = mma
		.meshs
		.add(Triangle::new(FULL_CIRCLE_RADIUS / 10., SMALLER_CIRCLE_RADIUS * 0.75).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center).translate_z(layer_counter),
			material: mma.mats.add(Color::BLUE.into()),
			..default()
		}
		.insert(NeedleStrength(thrust_type))
		.not_pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;

	// velocity
	let mesh = mma
		.meshs
		.add(Triangle::new(FULL_CIRCLE_RADIUS / 10., SMALLER_CIRCLE_RADIUS * 0.9).into());
	parent.spawn(
		MaterialMesh2dBundle {
			mesh: mesh.into(),
			transform: Transform::from_translation(circle_center).translate_z(layer_counter),
			material: mma.mats.add(Color::ORANGE.into()),
			..default()
		}
		.insert(NeedleVelocity(thrust_type))
		.not_pickable()
		.render_layer(BottomLeft),
	);
	layer_counter += 1.;
}

pub fn update_bottom_left_camera(
	In((velocity, relative_strength)): In<(
		Thrust<RelativeVelocityMagnitudes>,
		Thrust<RelativeStrength>,
	)>,
	mut needle_velocity: Query<(&NeedleVelocity, &mut Transform), Without<NeedleStrength>>,
	mut needle_force: Query<(&NeedleStrength, &mut Transform), Without<NeedleVelocity>>,
	mut input_flags: Query<(&InputFlag, &mut Handle<ColorMaterial>)>,

	player: Query<&MainPlayer>,

	mut mma: MM2,
) {
	let thrust_responses = &player.single().thrust_responses;

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
		mut material,
	) in input_flags.iter_mut()
	{
		*material = mma.mats.add(
			{
				thrust_responses
					.get_from_type(*thrust_type)
					.into_colour(*is_right)
			}
			.into(),
		);
	}
}

impl ThrustReactions {
	fn into_colour(&self, is_right: bool) -> Color {
		match self {
			ThrustReactions::Normal { input } => match (input, is_right) {
				(Some(true), true) | (Some(false), false) => USER_ENABLED_COL,
				(None, _) | (Some(false), true) | (Some(true), false) => DISABLED_INPUT_COL,
			},
			ThrustReactions::ArtificialFriction { friction_direction } => {
				match (friction_direction, is_right) {
					(Some(true), true) | (Some(false), false) => ARTIFICIAL_FRICTION_ENABLED_COL,
					(None, _) | (Some(false), true) | (Some(true), false) => DISABLED_INPUT_COL,
				}
			}
			ThrustReactions::Braking { braking_direction } => match (braking_direction, is_right) {
				(Some(true), true) | (Some(false), false) => BRAKING_ENABLED_COL,
				(None, _) | (Some(false), true) | (Some(true), false) => DISABLED_INPUT_COL,
			},
		}
	}
}
