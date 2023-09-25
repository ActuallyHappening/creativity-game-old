use crate::utils::*;

#[derive(Event, Debug, Deref)]
pub struct PlayerMinedPixel(Pixel);

impl PlayerMinedPixel {
	pub fn new(pixel: Pixel) -> Option<Self> {
		if pixel.player_mineable.is_some() {
			Some(Self(pixel))
		} else {
			None
		}
	}
}