use bevy::prelude::*;

use crate::utils::{*, Font};

mod macros;
use macros::*;
mod item_preview;
mod inventory;
use inventory::*;

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
					flex_grow: 1,
				},
				..default()
			}
			.named("Inventory UI")
			.not_pickable(),
		)
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: style! {Style
						flex_grow: 1,
						aspect_ratio: 1,
						border: 2 px,
						// width: 100 %,
						height: 100 %,
						max_height: 250 px,
					},
					background_color: Color::ALICE_BLUE.with_a(0.2).into(),
					border_color: Color::BLACK.into(),
					..default()
				})
				.with_children(|parent| {

				});

			parent
				.spawn(NodeBundle {
					style: style! {Style
						flex_grow: 5,
					},
					..default()
				})
				.with_children(|parent| {
					PlayerInventory::ui(parent, &mut mma);
				});
		});
}
