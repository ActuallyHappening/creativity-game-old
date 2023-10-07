use crate::utils::*;

use super::MainPlayer;

/// Not used directly as a Component, see [Weapon]
#[derive(Debug, Default, Component, Clone)]
pub struct WeaponFlags {
	/// edited from systems
	pub(self) try_fire_this_frame: Option<bool>,


}

pub fn should_fire_this_frame(
	keyboard: Res<Input<KeyCode>>,
) -> bool {
	keyboard.just_pressed(KeyCode::F)
}

pub fn toggle_fire(In(should_fire): In<bool>, player: Query<&mut MainPlayer>, mut weapons: Query<&mut Weapon>) {
	for mut weapon in weapons.iter_mut() {
		weapon.flags.try_fire_this_frame = Some(should_fire);
	}
}

pub fn handle_firing(mut weapons: Query<(&mut Weapon, &GlobalTransform)>, mut commands: Commands, mut mma: MM) {
	for (mut weapon, transform) in weapons.iter_mut() {
		if let Some(try_fire) = weapon.flags.try_fire_this_frame {
			weapon.flags.try_fire_this_frame = None;
			if try_fire {
				let transform = transform.reparented_to(&GlobalTransform::IDENTITY);
				info!("Firing weapon at: {:?}", transform);

				commands.spawn(PbrBundle {
					transform,
					material: mma.mats.add(Color::RED.into()),
					mesh: mma.meshs.add(shape::Capsule {
						radius: PIXEL_SIZE / 2.,
						depth: PIXEL_SIZE * 1.5,
						..default()
					}.into()),
					..default()
				});
			}
		}
	}
}