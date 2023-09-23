use bevy::prelude::*;

mod player;
mod setup;

pub struct MainPlugin;
impl Plugin for MainPlugin {
	fn build(&self, _app: &mut App) {}
}
