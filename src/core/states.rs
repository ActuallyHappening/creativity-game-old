use crate::utils::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, States)]
pub enum GameStates {
	#[default]
	PlayField,
	
	Designing,
}