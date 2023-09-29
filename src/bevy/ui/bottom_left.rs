use crate::utils::*;
use super::*;

pub fn setup_bottom_left_cam(mut commands: Commands, mut mma: MM2) {
	commands
		.spawn(UiCamera::<BottomLeft>::get_offset_bundle())
		.with_children(|parent| {
			// Circle
			parent.spawn(MaterialMesh2dBundle {
				mesh: mma.meshs.add(shape::Circle::new(100.).into()).into(),
				material: mma.mats.add(ColorMaterial::from(Color::PURPLE)),
				transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
				..default()
			});
		});
}