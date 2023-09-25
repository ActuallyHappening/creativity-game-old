use bevy::prelude::*;

use crate::utils::*;

mod macros;
use macros::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, ui)
			.add_systems(Update, update_inventory_ui);
	}
}

fn ui(mut commands: Commands, mut mma: MMA) {
	commands
		.spawn(
			(
				NodeBundle {
					// style: Style {
					// 	width: Val::Percent(100.0),
					// 	height: Val::Percent(100.0),
					// 	justify_content: JustifyContent::FlexEnd,
					// 	align_items: AlignItems::Center,
					// 	..default()
					// },
					style: style! { Style
						width: 100%,
						height: 100%,
						justify_content: center,
						align_items: end,
						margin: 10 px,
						flex_direction: column,
					},
					..default()
				},
				Name::from("Inventory UI"),
			)
				.not_pickable(),
		)
		.with_children(|parent| {
			PlayerInventory::ui(parent, &mut mma);
		});
}

#[derive(Component, Constructor)]
struct PlayerInventoryText {
	variant: PixelVariant
}

fn update_inventory_ui(mut invent_texts: Query<(&mut Text, &PlayerInventoryText)>, inventory: Res<PlayerInventory>) {
	// let copper_count = inventory[PixelVariant::Copper];

	// copper.single_mut().sections[1].value = format!("{copper_count}")

	for (mut text, PlayerInventoryText { variant }) in invent_texts.iter_mut() {
		if variant.default().collectable.is_some() {
			text.sections[1].value = format!("{}", inventory[*variant]);
		}
	}
}

impl PlayerInventory {
	fn ui(parent: &mut ChildBuilder, (_, _, ass): &mut MMA) {
		let text_style = TextStyle {
			font: ass.load("fonts/FiraMono-Medium.ttf"),
			font_size: 30.,
			color: Color::PURPLE,
		};

		for pixel in Pixel::iter_mineable() {
			parent.spawn(
				(
					TextBundle::from_sections([
						TextSection::new(format!("{}: ", pixel.name), text_style.clone()),
						TextSection::new("0", text_style.clone()),
					])
					.with_style(style! {Style
						margin: 5 px,
						// max_width: 100 px,
					}),
					Name::from(format!("{} count", pixel.name)),
					PlayerInventoryText::new(pixel.variant),
				)
					.not_pickable(),
			);
		}
	}
}
