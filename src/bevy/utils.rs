//! Various constants and utility types

use bevy::prelude::*;

#[allow(clippy::upper_case_acronyms)]
pub type MMA<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);

pub const PIXEL_SIZE: f32 = 5.;
pub const WORLD_WIDTH: usize = 100;
#[test]
fn world_width_is_even() {
	assert_eq!(WORLD_WIDTH % 2, 0);
}

pub const CAMERA_HEIGHT: f32 = 200.;
pub const LIGHT_HEIGHT: f32 = CAMERA_HEIGHT * 1.5;

#[extension_traits::extension(pub trait ColoursExt)]
impl Color {
	const BROWN: Color = Color::rgb(0.5, 0.25, 0.0);
}

#[derive(derive_more::Constructor)]
pub struct WorldPoint {
	/// player left-right
	pub x: i32,
	/// player up-down
	pub y: i32,
	/// player backwards-forwards
	pub z: i32,
}

impl WorldPoint {
	pub fn into_bevy_vector(self) -> Vec3 {
		Vec3::new(
			self.x as f32 * PIXEL_SIZE,
			self.y as f32 * PIXEL_SIZE,
			self.z as f32 * PIXEL_SIZE,
		)
	}
}
