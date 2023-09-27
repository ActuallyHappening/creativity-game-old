use crate::utils::*;

#[derive(Event, Debug, Deref)]
pub struct PlayerMinedPixel(Pixel);

impl PlayerMinedPixel {
	pub fn new(pixel: Pixel) -> Option<Self> {
		if pixel.collectable.is_some() {
			Some(Self(pixel))
		} else {
			None
		}
	}
}
