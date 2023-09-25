use std::collections::HashMap;
use bevy::prelude::*;
use crate::utils::*;

use super::pixels::PixelVariant;


#[derive(Resource, Debug)]
pub struct PlayerInventory(HashMap<PixelVariant, u32>);

impl Default for PlayerInventory {
	fn default() -> Self {
		let mut invent = HashMap::new();
		for variant in PixelVariant::iter() {
			invent.insert(variant, 0);
		}
		PlayerInventory(invent)
	}
}

impl std::ops::Deref for PlayerInventory {
	type Target = HashMap<PixelVariant, u32>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
