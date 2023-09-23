use bevy::prelude::*;
use bevy_mod_picking::{
	prelude::{RaycastPickCamera, RaycastPickTarget, DefaultHighlightingPlugin, DebugPickingPlugin},
	PickableBundle, DefaultPickingPlugins,
};

const CAMERA_HEIGHT: f32 = 100.;
const LIGHT_HEIGHT: f32 = 100.;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_plugins(
			DefaultPickingPlugins
				.build()
				.disable::<DefaultHighlightingPlugin>()
				.disable::<DebugPickingPlugin>(),
		);
	}
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// cam
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0., CAMERA_HEIGHT, 0.)
				.with_rotation(Quat::from_rotation_x(-90_f32.to_radians())),
			..default()
		},
		RaycastPickCamera::default(),
		MainCamera,
	));

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 50000.0,
			range: 250.,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(0., LIGHT_HEIGHT, 0.),
		..default()
	});

	// // ground plane
	// commands.spawn((
	// 	PbrBundle {
	// 		mesh: meshes.add(shape::Plane::from_size(500.0).into()),
	// 		material: materials.add(Color::SILVER.into()),
	// 		// transform to be behind, xy plane
	// 		transform: Transform::from_xyz(0., 0., 0.),
	// 		..default()
	// 	},
	// 	PickableBundle::default(),    // Makes the entity pickable
	// 	RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
	// ));
}
