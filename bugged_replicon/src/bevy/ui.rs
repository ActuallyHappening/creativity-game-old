use crate::utils::*;

use bevy::app::{PluginGroup, PluginGroupBuilder};

mod startscreen;
pub use startscreen::StartScreenPlugin;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
	fn build(self) -> bevy::app::PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(StartScreenPlugin)
	}
}
