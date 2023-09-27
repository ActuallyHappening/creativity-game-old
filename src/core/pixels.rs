use crate::utils::*;

mod structures;
pub use structures::*;

/// Data about a class of pixels
#[derive(Component, Debug, Clone)]
pub struct Pixel {
	pub name: &'static str,
	pub description: &'static str,
	pub colour: Color,
	pub variant: PixelVariant,
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

pub struct PixelVariantInfo {
	pub collectable: Option<Collect>,
	pub naturally_spawning: Option<Natural>,
}

impl PixelVariant {
	// todo: maybe put this info in a lazy_static and reference it instead of re-constructing it everywhere?
	const fn default_hardcoded(self) -> (Pixel, PixelVariantInfo) {
		type PV = PixelVariant;
		match self {
			PV::Dirt => (
				Pixel {
					name: "Dirt",
					description: "A block of dirt",
					colour: Color::rgb(0.3, 0.25, 0.0),
					variant: self,
				},
				PixelVariantInfo {
					collectable: None,
					naturally_spawning: Some(Natural { frequency: 1000 }),
				},
			),
			PV::Copper => (
				Pixel {
					name: "Copper",
					description: "A block of copper",
					colour: Color::rgb(0.6, 0.25, 0.05),
					variant: self,
				},
				PixelVariantInfo {
					collectable: Some(Collect {
						amount_multiplier: 5,
						player_mineable: true,
					}),
					naturally_spawning: Some(Natural { frequency: 150 }),
				},
			),
			PV::Lead => (
				Pixel {
					name: "Lead",
					description: "A block of lead",
					colour: Color::SILVER,
					variant: self,
				},
				PixelVariantInfo {
					collectable: Some(Collect {
						amount_multiplier: 1,
						player_mineable: true,
					}),
					naturally_spawning: Some(Natural { frequency: 3 }),
				},
			),
		}
	}

	pub const fn get_default_pixel(self) -> Pixel {
		self.default_hardcoded().0
	}

	pub const fn get_variant_info(self) -> PixelVariantInfo {
		self.default_hardcoded().1
	}

	pub fn natural_pool() -> Vec<(PixelVariant, Natural)> {
		let mut pool = Vec::new();
		for variant in Self::iter() {
			if let Some(natural) = variant.get_variant_info().naturally_spawning {
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

	/// Returns all variants that are mineable by the player
	pub fn get_mineable_variants() -> impl Iterator<Item = PixelVariant> {
		PixelVariant::iter().filter(|v| v.get_variant_info().is_player_mineable())
	}
}

impl PixelVariantInfo {
	pub fn is_player_mineable(&self) -> bool {
		self
			.collectable
			.as_ref()
			.is_some_and(|collect| collect.player_mineable)
	}
}
