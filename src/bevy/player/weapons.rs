use crate::utils::*;

use super::MainPlayer;

#[derive(Debug, Default, Component, Clone)]
pub struct WeaponFlags {
	/// edited from systems
	pub(self) try_fire_this_frame: bool,


}

pub fn should_fire_this_frame(
	keyboard: Res<Input<KeyCode>>,
) -> bool {
	keyboard.just_pressed(KeyCode::F)
}

pub fn toggle_fire(In(should_fire): In<bool>, player: Query<&mut MainPlayer>) {
	
}

pub fn handle_firing() {
	
}