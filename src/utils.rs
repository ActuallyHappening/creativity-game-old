//! Various constants and utility types

#![allow(dead_code)]

use bevy::ecs::system::SystemParam;
use bevy_mod_picking::{
	prelude::{Pickable, RaycastPickTarget},
	PickableBundle,
};

pub use crate::bevy::types::*;
pub use crate::core::Direction;
pub use crate::core::*;
pub use bevy::prelude::*;
pub use bevy::sprite::MaterialMesh2dBundle;
pub use bevy::{
	core_pipeline::clear_color::ClearColorConfig, render::camera::Viewport, window::WindowResized,
};
pub use bevy_dolly::prelude::*;
pub use bevy_mod_picking::prelude::*;
pub use bevy_rapier3d::prelude::*;
pub use contracts::*;
pub use derive_builder::Builder;
pub use derive_more::{Deref, DerefMut};
pub use derive_more::*;
pub use extension_traits::extension;
pub use rand::{random, Rng};
pub use static_assertions::*;
pub use std::any;
pub use std::borrow::Cow;
pub use std::f32::consts::{PI, TAU};
pub use std::{
	marker::PhantomData,
	ops::{Add, Div, Mul},
};
pub use strum::*;
pub use std::collections::HashMap;
pub use std::num::*;

#[cfg(feature = "hanabi_particles")]
mod particles;
#[cfg(feature = "hanabi_particles")]
pub use bevy_hanabi::*;
#[cfg(feature = "hanabi_particles")]
pub use particles::*;

mod triangle;
pub use triangle::*;

mod test;

mod text;
pub use text::Text2dBundle;

#[allow(clippy::upper_case_acronyms)]
#[derive(SystemParam)]
pub struct MMA<'w> {
	pub meshs: ResMut<'w, Assets<Mesh>>,
	pub mats: ResMut<'w, Assets<StandardMaterial>>,
	pub ass: Res<'w, AssetServer>,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(SystemParam)]
pub struct MM<'w> {
	pub meshs: ResMut<'w, Assets<Mesh>>,
	pub mats: ResMut<'w, Assets<StandardMaterial>>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(SystemParam)]
pub struct MMA2<'w> {
	pub meshs: ResMut<'w, Assets<Mesh>>,
	pub mats: ResMut<'w, Assets<ColorMaterial>>,
	pub ass: Res<'w, AssetServer>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(SystemParam)]
pub struct MM2<'w> {
	pub meshs: ResMut<'w, Assets<Mesh>>,
	pub mats: ResMut<'w, Assets<ColorMaterial>>,
}

pub const PIXEL_SIZE: f32 = 5.;
pub const WORLD_WIDTH: usize = 20;
#[test]
fn world_width_is_even() {
	assert_eq!(WORLD_WIDTH % 2, 0);
}

pub const CAMERA_HEIGHT: f32 = 100.;
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
	fn physics_zero_damping(self) -> (Damping, Self) {
		self.insert(Damping {
			linear_damping: 0.,
			angular_damping: 0.,
		})
	}
	fn physics_never_sleep(self) -> (Sleeping, Self) {
		self.insert(Sleeping::disabled())
	}
}

#[extension(pub trait TransformExt)]
impl Transform {
	fn translate_x(mut self, delta_x: f32) -> Self {
		self.translation.x += delta_x;
		self
	}

	fn translate_y(mut self, delta_y: f32) -> Self {
		self.translation.y += delta_y;
		self
	}

	fn translate_z(mut self, delta_z: f32) -> Self {
		self.translation.z += delta_z;
		self
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

pub fn join2<A, B, AMarker, BMarker>(
	mut a: A,
	mut b: B,
) -> impl FnMut(In<A::In>, ParamSet<(A::Param, B::Param)>) -> (A::Out, B::Out)
where
	A: SystemParamFunction<AMarker>,
	B: SystemParamFunction<BMarker, In = A::In>,
	A::In: Copy,
{
	move |In(input), mut params| {
		let out_a = a.run(input, params.p0());
		let out_b = b.run(input, params.p1());
		(out_a, out_b)
	}
}

pub fn join3<A, B, C, AMarker, BMarker, CMarker>(
	mut a: A,
	mut b: B,
	mut c: C,
) -> impl FnMut(In<A::In>, ParamSet<(A::Param, B::Param, C::Param)>) -> (A::Out, B::Out, C::Out)
where
	A: SystemParamFunction<AMarker>,
	B: SystemParamFunction<BMarker, In = A::In>,
	C: SystemParamFunction<CMarker, In = A::In>,
	A::In: Copy,
{
	move |In(input), mut params| {
		let out_a = a.run(input, params.p0());
		let out_b = b.run(input, params.p1());
		let out_c = c.run(input, params.p2());
		(out_a, out_b, out_c)
	}
}

pub fn sequence<A, B, AMarker, BMarker>(
	mut a_in: A,
	mut b_out: B,
) -> impl FnMut(In<A::In>, ParamSet<(A::Param, B::Param)>) -> B::Out
where
	A: SystemParamFunction<AMarker>,
	B: SystemParamFunction<BMarker, In = A::Out>,
{
	move |In(input), mut params| {
		let value = a_in.run(input, params.p0());
		b_out.run(value, params.p1())
	}
}

// pub fn init_debug_tools() {
// 	#[cfg(not(target_arch = "wasm32"))]
// 	tracing_subscriber::fmt::init();

// 	#[cfg(target_arch = "wasm32")]
// 	{
// 		use tracing_subscriber::prelude::*;
// 		console_error_panic_hook::set_once();
// 		tracing_subscriber::registry::Registry::default()
// 			.with(tracing_wasm::WASMLayer::new(
// 				tracing_wasm::WASMLayerConfig::default(),
// 			))
// 			.init();
// 	}
// }
