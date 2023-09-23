use bevy::prelude::*;

pub type MMA<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);

pub const PIXEL_SIZE: f32 = 5.;