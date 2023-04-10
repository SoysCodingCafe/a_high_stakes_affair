use bevy::prelude::*;

use super::food::DropType;

pub const PEG_DEPTH: f32 = -1.0;

#[derive(Component, Default, Clone, Copy)]
pub struct Peg(pub PegType);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum PegType {
	#[default]
	PachinkoPeg,
	ItemPeg(DropType),
}
