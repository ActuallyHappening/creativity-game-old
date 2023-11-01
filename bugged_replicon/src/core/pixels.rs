use crate::utils::*;

mod structures;
pub use structures::*;
mod macros;
use macros::*;

/// Data about a class of pixels
/// Does not implement [PartialEq] because the identity of a pixel is only in its variant,
/// spawning default pixels does not imply that all default pixels are the same,
/// even though all of the information contained within this struct would imply that
/// [PartialEq] they are equal.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Pixel {
	pub name: Cow<'static, str>,
	pub description: Cow<'static, str>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Serialize, Deserialize)]
pub enum PixelVariant {
	Dirt,
	Copper,
	Lead,

	/// Used to create player
	PlayerSteel,
	/// Used for player engine
	PlayerLargeEngineDecoration,
}

pub struct PixelVariantInfo {
	pub collectable: Option<Collect>,
	pub naturally_spawning: Option<Natural>,
}