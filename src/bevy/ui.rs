use crate::utils::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup_camera::<BottomLeft>)
			.add_systems(Update, update_camera::<BottomLeft>);
	}
}

mod camtype;
pub use camtype::*;

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
		info!("Setting cam translation to {:?}", cam_translation);
		cam.translation = Vec3::new(cam_translation.x as f32, cam_translation.y as f32, 0.);
	}
}