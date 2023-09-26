use bevy::prelude::{ChildBuilder, NodeBundle};

use crate::utils::*;
use super::*;

/// A square [ItemPreview::WIDTH_PERCENT] of the screen
pub struct ItemPreview;

impl ItemPreview {
	/// Width of viewport taken by item preview
	const WIDTH_PERCENT: f32 = 0.2;

	/// Flex location of preview, assuming placed in top right of screen
	/// with no margin between this and viewport border
	pub fn ui(parent: &mut ChildBuilder) {
		// ui for preview with border
		parent
			.spawn(NodeBundle {
				style: style! {Style 
					aspect_ratio: 1,
					// border: 5 px,
				}.with_width_vw(Self::WIDTH_PERCENT),
				// border_color: Color::BLACK.into(),
				background_color: Color::PURPLE.into(),
				..default()
			})
			.not_pickable();
	}
}
