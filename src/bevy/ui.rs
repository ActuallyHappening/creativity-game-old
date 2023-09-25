use bevy::prelude::*;

use super::{player::PlayerInventory, utils::BundleExt};

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, ui).add_systems(Update, update_inventory_ui);
	}
}

fn ui(mut commands: Commands, ass: Res<AssetServer>) {
	commands
		.spawn((
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			},
			Name::from("Inventory UI"),
		).not_pickable())
		.with_children(|parent| {
			parent.spawn((
				TextBundle::from_section(
					"Copper count: ",
					TextStyle {
						font: ass.load("fonts/FiraMono-Medium.ttf"),
						font_size: 30.,
						color: Color::PURPLE,
					},
				)
				.with_style(Style {
					margin: UiRect::top(Val::Px(15.)),
					..default()
				}),
				Name::from("Copper count"),
			).not_pickable());
		});
}

fn update_inventory_ui(mut copper: Query<&mut Text, With<Name>>, inventory: Res<PlayerInventory>) {
	let copper_count = inventory.copper;

	copper.single_mut().sections[0].value = format!("Copper count: {copper_count}")
}