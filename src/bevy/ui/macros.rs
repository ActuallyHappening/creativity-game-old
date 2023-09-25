#![allow(unused_imports)]
use crate::utils::*;
use bevy::{prelude::Style, ui::UiRect};

macro_rules! to_literal_f32 {
	($val:literal) => {
		stringify!($val.0)
			.parse()
			.unwrap_or(stringify!($val).parse().unwrap())
	};
}

pub(crate) use to_literal_f32;

macro_rules! style {
		( first $($rest:tt)+ ) => {
			style!{
				prev
					Style {
						..default()
					}
				;next; $($rest)*
			}
		};

		// margin
		(prev $prev:expr ;next; margin: $val:literal px, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.margin = UiRect::all(Val::Px(to_literal_f32!($val)));
					prev
				} ;next; $($rest)*
			}
		};

		// generic % percentages
		(prev $prev:expr ;next; $prop:ident: $val:literal %, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.$prop = Val::Percent(to_literal_f32!($val));
					prev
				} ;next; $($rest)*
			}
		};
		// generic px pixels
		(prev $prev:expr ;next; $prop:ident: $val:literal px, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.$prop = Val::Px(to_literal_f32!($val));
					prev
				} ;next; $($rest)*
			}
		};

		// justify content
		(prev $prev:expr ;next; justify-content: center, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.justify_content = JustifyContent::Center;
					prev
				} ;next; $($rest)*
			}
		};
		(prev $prev:expr ;next; justify-content: end, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.justify_content = JustifyContent::End;
					prev
				} ;next; $($rest)*
			}
		};
		(prev $prev:expr ;next; justify-content: start, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.justify_content = JustifyContent::Start;
					prev
				} ;next; $($rest)*
			}
		};
		// align items
		(prev $prev:expr ;next; align-items: center, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.align_items = AlignItems::Center;
					prev
				} ;next; $($rest)*
			}
		};
		(prev $prev:expr ;next; align-items: end, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.align_items = AlignItems::End;
					prev
				} ;next; $($rest)*
			}
		};
		(prev $prev:expr ;next; align-items: start, $($rest:tt)*) => {
			style!{
				prev {
					let mut prev = $prev;
					prev.align_items = AlignItems::Start;
					prev
				} ;next; $($rest)*
			}
		};

		// base case
		(prev $prev:expr ;next; ) => {
			$prev
		};
}

#[test]
fn test_style_macro() {
	assert_eq!(
		style! {first width: 100%, },
		Style {
			width: Val::Percent(100.),
			..default()
		}
	);
	assert_eq!(
		style! {first height: 69%, },
		Style {
			height: Val::Percent(69.),
			..default()
		}
	);
	assert_eq!(
		style! {first
			justify-content: center,
			justify-content: end,
			justify-content: start,
		},
		Style {
			justify_content: JustifyContent::Start,
			..default()
		}
	);

	assert_eq!(
		style! {first
			align-items: center,
			align-items: end,
			align-items: start,
		},
		Style {
			align_items: AlignItems::Start,
			..default()
		}
	);

	assert_eq!(
		style! {first margin: 69 px,},
		Style {
			margin: UiRect::all(Val::Px(69.)),
			..default()
		}
	)
}

pub(crate) use style;
