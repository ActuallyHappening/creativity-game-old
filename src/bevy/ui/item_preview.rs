use bevy::{render::camera::Viewport, window::WindowResized, core_pipeline::clear_color::ClearColorConfig};

use crate::bevy::camera::MainCamera;

use super::*;

/// A square [ItemPreview::WIDTH_PERCENT] of the screen
pub struct ItemPreview;
impl Plugin for ItemPreview {
	fn build(&self, app: &mut App) {
			app.add_systems(Startup, |mut commands: Commands| {
				commands.spawn(Camera3dBundle {
					transform: Transform::from_xyz(0., CAMERA_HEIGHT, 0.),
					camera: Camera {
						order: 1,
						..default()
					},
					camera_3d: Camera3d {
						clear_color: ClearColorConfig::None,
						..default()
					},
					..default()
				}.insert(PreviewCamera)).not_pickable();
			}).add_systems(Update, update_preview_cam);
	}
}

#[derive(Component)]
struct PreviewCamera;

lazy_static::lazy_static! {
	static ref INITIAL_ROT: Quat = Quat::from_rotation_x(-90f32.to_radians());
}

impl ItemPreview {
	/// Width of viewport taken by item preview
	const WIDTH_PERCENT: f32 = 20.;
	const MARGIN: f32 = 0.;

	/// Flex location of preview, assuming placed in top right of screen
	/// with no margin between this and viewport border
	pub fn ui(parent: &mut ChildBuilder) {
		// ui for preview with border
		parent
			.spawn(
				NodeBundle {
					style: style! {Style
						// aspect_ratio: 1,
						border: 0 px,
						margin: 0 px,
					}
					.with_width_vw(Self::WIDTH_PERCENT)
					.with_height_vw(Self::WIDTH_PERCENT),
					border_color: Color::BLACK.into(),
					// background_color: Color::PURPLE.into(),
					..default()
				}
				.named("Item Preview UI"),
			)
			.not_pickable();
	}
}

fn update_preview_cam(
	windows: Query<&Window>,
	mut resize_events: EventReader<WindowResized>,
	mut cam: Query<&mut Camera, (With<PreviewCamera>, Without<MainCamera>)>,
) {
	// We need to dynamically resize the camera's viewports whenever the window size changes
	// A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
	for resize_event in resize_events.iter() {
		let window = windows.get(resize_event.window).unwrap();
		let mut preview_cam = cam.single_mut();

		let width = window.resolution.physical_width() as f32;
		let _height = window.resolution.physical_height() as f32;
		let preview_width = (ItemPreview::WIDTH_PERCENT / 100.) * width;
		let top_left_x = width - ItemPreview::MARGIN * 2. - preview_width;
		let top_left_y = 0. - ItemPreview::MARGIN;

		preview_cam.viewport = Some(Viewport {
			physical_position: UVec2::new(top_left_x as u32, top_left_y as u32),
			physical_size: UVec2::new(preview_width as u32, preview_width as u32),
			..default()
		});

		// let mut right_camera = right_camera.single_mut();
		// right_camera.viewport = Some(Viewport {
		//     physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
		//     physical_size: UVec2::new(
		//         window.resolution.physical_width() / 2,
		//         window.resolution.physical_height(),
		//     ),
		//     ..default()
		// });
	}
}
