use bevy::{
	prelude::*
};

use crate::fiber::framerate::FrameRate;


use bevy::render::color::Color;

#[non_exhaustive]
pub struct UiLight;
impl UiLight{
	pub const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
	pub const HOVERED_BUTTON: Color = Color::rgb(0.85, 0.85, 0.85);
	pub const PRESSED_BUTTON: Color = Color::rgb(0.65, 0.65, 0.65);
}

#[non_exhaustive]
pub struct UiDark;
impl UiDark {
	const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
	const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
	const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
}

pub struct UiTree {
	background: NodeBundle,
	background_style: Style,

}

pub struct Ui;

fn ui_setup(mut _commands: Commands) {
	_commands.spawn(Camera2dBundle::default());
}


impl Plugin for Ui {
	fn build(&self, app: &mut App) {
		app.add_plugin(FrameRate)
		.add_startup_system(ui_setup);
	}
}