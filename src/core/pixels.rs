use crate::utils::*;
use strum::EnumIter;

/// Data about a class of pixels
#[derive(Component, Debug, Clone)]
pub struct Pixel {
	pub name: &'static str,
	pub description: &'static str,
	pub colour: Color,
	pub variant: PixelVariant,

	pub collectable: Option<Collect>,
	pub naturally_spawning: Option<Natural>,
}

#[derive(Debug, Clone)]
pub struct Natural {
	/// Higher the number, greater chance of spawning
	pub frequency: u16,
}

#[derive(Debug, Clone)]
pub struct Collect {
	pub player_mineable: bool,
	pub amount_multiplier: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PixelVariant {
	Dirt,
	Copper,
	Lead,
}

impl PixelVariant {
	pub fn default(self) -> Pixel {
		type PV = PixelVariant;
		match self {
			PV::Dirt => Pixel {
				name: "Dirt",
				description: "A block of dirt",
				colour: Color::rgb(0.3, 0.25, 0.0),
				collectable: None,
				naturally_spawning: Some(Natural { frequency: 1000 }),
				variant: self,
			},
			PV::Copper => Pixel {
				name: "Copper",
				description: "A block of copper",
				colour: Color::rgb(0.6, 0.25, 0.05),
				collectable: Some(Collect {
					amount_multiplier: 5,
					player_mineable: true,
				}),
				naturally_spawning: Some(Natural { frequency: 150 }),
				variant: self,
			},
			PV::Lead => Pixel {
				name: "Lead",
				description: "A block of lead",
				colour: Color::SILVER,
				collectable: Some(Collect {
					amount_multiplier: 1,
					player_mineable: true,
				}),
				naturally_spawning: Some(Natural { frequency: 3 }),
				variant: self,
			},
		}
	}

	#[ensures(ret.len() == 3)]
	pub fn natural_pool() -> Vec<(PixelVariant, Natural)> {
		let mut pool = Vec::new();
		for variant in Self::iter() {
			if let Some(natural) = variant.default().naturally_spawning {
				pool.push((variant, natural));
			}
		}
		pool
	}
}

impl PixelVariant {
	pub fn iter() -> impl Iterator<Item = PixelVariant> {
		<PixelVariant as strum::IntoEnumIterator>::iter()
	}
}

impl Pixel {
	pub fn iter() -> impl Iterator<Item = Pixel> {
		PixelVariant::iter().map(|variant| variant.default())
	}

	pub fn iter_mineable() -> impl Iterator<Item = Pixel> {
		Self::iter().filter(|pixel| {
			pixel
				.collectable
				.as_ref()
				.is_some_and(|p| p.player_mineable.clone())
		})
	}
}
