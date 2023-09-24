//! Various constants and utility types


use bevy::prelude::*;

#[allow(clippy::upper_case_acronyms)]
pub type MMA<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);

pub const PIXEL_SIZE: f32 = 5.;

pub const CAMERA_HEIGHT: f32 = 200.;
pub const LIGHT_HEIGHT: f32 = CAMERA_HEIGHT * 1.5;

#[extension_traits::extension(pub trait ColoursExt)]
impl Color {
	const BROWN: Color = Color::rgb(0.5, 0.25, 0.0);
}