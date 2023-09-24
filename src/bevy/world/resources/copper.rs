use bevy::prelude::Color;
use rand::Rng;

use super::traits::impl_pixel_type;
use crate::bevy::utils::*;

#[derive(rand_derive::Rand)]
pub struct RCopper {
	capacity: u32,
}

impl_pixel_type!(
	RCopper {
		col = Color::BROWN,
		res = { cap = { range = 0..=100  }},
		natural = { freq = 30, random = rand::thread_rng().gen()},
	}
);
