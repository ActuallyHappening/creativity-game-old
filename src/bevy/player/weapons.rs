use crate::utils::*;

use super::MainPlayer;

#[derive(Debug, Default)]
pub struct WeaponFlags {
	fire_this_frame: bool,
}

pub fn should_fire_this_frame(
	keyboard: Res<Input<KeyCode>>,
) -> bool {
	keyboard.just_pressed(KeyCode::F)
}

pub fn fire(In(should_fire): In<bool>, player: Query<&mut MainPlayer>) {
	
}