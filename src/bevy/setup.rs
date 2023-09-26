use bevy::prelude::*;
use bevy_mod_picking::{
	prelude::{DebugPickingPlugin, DefaultHighlightingPlugin},
	DefaultPickingPlugins,
};
#[cfg(feature = "dev")]
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use super::{camera::CameraPlugin, world_gen::spawn_random_world};
use crate::utils::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Msaa::default())
			.add_systems(Startup, (setup, spawn_random_world))
			.add_plugins(
				DefaultPickingPlugins
					.build()
					.disable::<DefaultHighlightingPlugin>()
					.disable::<DebugPickingPlugin>(),
			);

		#[cfg(feature = "dev")]
		app.add_plugins((
			ScreenFrameDiagnosticsPlugin,
			ScreenDiagnosticsPlugin::default(),
		));

		app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
		app.add_systems(Startup, |mut physics_config: ResMut<RapierConfiguration>| {
			physics_config.gravity = Vec3::ZERO;
		});

		#[cfg(feature = "debugging")]
		app.add_plugins(RapierDebugRenderPlugin::default());
	}
}

pub fn setup(
	mut commands: Commands,
) {
	// cam
	commands.spawn(CameraPlugin::default());

	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 5000000.0,
			range: 25000.,
			// shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(0., LIGHT_HEIGHT, 0.),
		..default()
	});
}
