use bevy::render::view::RenderLayers;

use crate::utils::*;

#[derive(Debug)]
pub enum Layer {
	UiBottomLeft,
	UiTopLeft,
	UiTopRight,
	UiBottomRight,

	Stars,
}

impl From<Layer> for RenderLayers {
	fn from(value: Layer) -> Self {
		match value {
			Layer::UiBottomLeft => RenderLayers::none().with(30),
			Layer::UiTopLeft => RenderLayers::none().with(29),
			Layer::UiTopRight => RenderLayers::none().with(28),
			Layer::UiBottomRight => RenderLayers::none().with(27),
			Layer::Stars => RenderLayers::none().with(1),
		}
	}
}

#[extension(pub trait BundleExt2)]
impl<T: Bundle> T {
	fn layer(self, layer: Layer) -> (RenderLayers, Self) {
		self.insert(layer.into())
	}
}
