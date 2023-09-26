//! Various constants and utility types

use bevy::prelude::*;
use bevy_mod_picking::{
	prelude::{Pickable, RaycastPickTarget},
	PickableBundle,
};
use bevy_rapier3d::rapier::prelude::RigidBodyType;
use extension_traits::extension;

pub use crate::core::*;
pub use bevy::prelude::*;
pub use bevy_mod_picking::prelude::*;
pub use bevy_rapier3d::prelude::*;
pub use contracts::*;
pub use derive_more::Deref;
pub use derive_more::*;

#[allow(clippy::upper_case_acronyms)]
pub type MMA<'a> = (
	ResMut<'a, Assets<Mesh>>,
	ResMut<'a, Assets<StandardMaterial>>,
	ResMut<'a, AssetServer>,
);

pub const PIXEL_SIZE: f32 = 5.;
pub const WORLD_WIDTH: usize = 20;
#[test]
fn world_width_is_even() {
	assert_eq!(WORLD_WIDTH % 2, 0);
}

pub const CAMERA_HEIGHT: f32 = 200.;
pub const LIGHT_HEIGHT: f32 = CAMERA_HEIGHT * 1.5;

#[extension(pub trait ColoursExt)]
impl Color {
	const BROWN: Color = Color::rgb(0.5, 0.25, 0.0);
}

#[extension(pub trait BundleExt)]
impl<T: Bundle> T {
	fn pickable(self) -> (PickableBundle, RaycastPickTarget, Self) {
		(
			PickableBundle::default(),
			RaycastPickTarget::default(),
			self,
		)
	}

	fn not_pickable(self) -> (Pickable, Self) {
		(Pickable::IGNORE, self)
	}

	fn named(self, name: impl Into<std::borrow::Cow<'static, str>>) -> (Name, Self) {
		(Name::new(name), self)
	}

	fn insert<B: Bundle>(self, bundle: B) -> (B, Self) {
		(bundle, self)
	}

	// physics
	fn physics_dynamic(self) -> (RigidBody, Self) {
		self.insert(RigidBody::Dynamic)
	}
	fn physics_collider_ball(self, size: f32) -> (Collider, Self) {
		self.insert(Collider::ball(size))
	}
	fn physics_restitution(self, coefficient: f32) -> (Restitution, Self) {
		self.insert(Restitution::coefficient(coefficient))
	}
	fn physics_zero_force(self) -> (ExternalForce, Self) {
		self.insert(ExternalForce {
			force: Vec3::ZERO,
			torque: Vec3::ZERO,
		})
	}
	fn physics_zero_velocity(self) -> (Velocity, Self) {
		self.insert(Velocity {
			linvel: Vec3::ZERO,
			angvel: Vec3::ZERO,
		})
	}
}

#[derive(derive_more::Constructor, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub enum Font {
	Medium,
}

impl Font {
	pub const fn into_path(self) -> &'static str {
		match self {
			Self::Medium => "fonts/FiraMono-Medium.ttf",
		}
	}
}

#[extension(pub trait StyleExt)]
impl Style {
	fn with_width_vw(mut self, vw: impl Into<f32>) -> Self {
		self.width = Val::Vw(vw.into());
		self
	}

	fn with_height_vh(mut self, vh: impl Into<f32>) -> Self {
		self.height = Val::Vh(vh.into());
		self
	}

	fn with_height_vw(mut self, vh: impl Into<f32>) -> Self {
		self.height = Val::Vw(vh.into());
		self
	}
}

pub fn init_debug_tools() {
	#[cfg(not(target_arch = "wasm32"))]
	tracing_subscriber::fmt::init();

	#[cfg(target_arch = "wasm32")]
	{
		use tracing_subscriber::prelude::*;
		console_error_panic_hook::set_once();
		tracing_subscriber::registry::Registry::default()
			.with(tracing_wasm::WASMLayer::new(
				tracing_wasm::WASMLayerConfig::default(),
			))
			.init();
	}
}
