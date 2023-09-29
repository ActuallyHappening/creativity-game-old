use crate::utils::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(
				Startup,
				(
					setup_camera::<BottomLeft>, //.in_set(BottomLeft),
					setup_bottom_left_cam,      //.after(BottomLeft),
					setup_camera::<TopLeft>.in_set(TopLeft),
					setup_top_left_cam.after(TopLeft),
					setup_camera::<TopRight>.in_set(TopRight),
					setup_top_right_cam.after(TopRight),
					setup_camera::<BottomRight>.in_set(BottomRight),
					setup_bottom_right_cam.after(BottomRight),
				),
			)
			.add_systems(
				Update,
				(
					update_camera::<BottomLeft>.in_set(BottomLeft),
					join3(
						sequence(
							get_base_normal_vectors,
							calculate_relative_velocity_magnitudes,
						),
						get_current_relative_strengths,
						get_current_braking_info,
					)
					.pipe(update_bottom_left_camera)
					.after(PlayerMove),
					update_camera::<TopLeft>,
					update_camera::<TopRight>,
					update_camera::<BottomRight>,
				),
			);
	}
}

mod camtype;
pub use camtype::*;
mod bottom_left;
use bottom_left::*;

use super::player::{
	calculate_relative_velocity_magnitudes, get_base_normal_vectors, get_current_braking_info,
	get_current_relative_strengths, PlayerMove,
};

fn setup_camera<T: CamType>(mut commands: Commands) {
	commands.spawn(UiCamera::<T>::get_camera_bundle());
}

fn update_camera<T: CamType>(
	windows: Query<&Window>,
	mut resize_events: EventReader<WindowResized>,
	mut cam: Query<&mut Transform, With<UiCamera<T>>>,
) {
	for ev in resize_events.iter() {
		let window = windows.get(ev.window).unwrap();
		let mut cam = cam.single_mut();

		let width = window.resolution.width();
		let height = window.resolution.height();

		let cam_translation = T::get_cam_transform(width / 2., height / 2.);
		// info!("Setting cam translation to {:?}", cam_translation);
		cam.translation = Vec3::new(cam_translation.x as f32, cam_translation.y as f32, 0.);
	}
}

fn setup_top_left_cam() {}
fn setup_top_right_cam() {}
fn setup_bottom_right_cam() {}
