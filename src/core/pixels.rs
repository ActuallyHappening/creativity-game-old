use bevy::prelude::*;
use strum::EnumIter;
use crate::utils::*;

/// Data about a class of pixels
#[derive(Debug)]
pub struct Pixel {
	pub name: &'static str,
	pub description: &'static str,
	pub colour: Color,

	pub player_mineable: Option<PlayerMineable>,
	pub naturally_spawning: Option<Natural>,
}

#[derive(Debug)]
pub struct Natural {
	/// Higher the number, greater chance of spawning
	pub frequency: u8,
}

#[derive(Debug)]
pub struct PlayerMineable {
	amount_multiplier: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PixelVariant {
	Dirt,
	Copper,
}

impl PixelVariant {
	fn default(self) -> Pixel {
		type PV = PixelVariant;
		match self {
			PV::Dirt => Pixel {
				name: "Dirt",
				description: "A block of dirt",
				colour: Color::rgb(0.5, 0.25, 0.0),
				player_mineable: None,
				naturally_spawning: Some(Natural { frequency: 100 }),
			},
			PV::Copper => Pixel {
				name: "Copper",
				description: "A block of copper",
				colour: Color::rgb(0.5, 0.25, 0.0),
				player_mineable: Some(PlayerMineable { amount_multiplier: 2 }),
				naturally_spawning: Some(Natural { frequency: 10 }),
			},
		}
	}

	pub fn natural_pool() -> Vec<(PixelVariant, Natural)> {
		let mut pool = Vec::new();
		for variant in Self::iter() {
			if let Some(natural) = variant.default().naturally_spawning {
				pool.push((variant, natural));
			}
		}
		pool
	}

	pub fn iter() -> impl Iterator<Item = PixelVariant> {
		<PixelVariant as strum::IntoEnumIterator>::iter()
	}
}