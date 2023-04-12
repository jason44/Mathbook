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

fn ui_setup(mut _commands: Commands) {
	_commands.spawn(Camera2dBundle::default());
}

pub struct Ui {
	primary_node: NodeBundle,
	side_pane: NodeBundle,
	side_pane_style: Style,
	header: NodeBundle,
	header_style: Style,
	input_box: NodeBundle,
	input_box_style: Style,							
}
#[derive(Component)]
struct FuncInputBox;

#[derive(Component)]
struct MatrixInputBox;

#[derive(Component)]
struct AnimateButton;

impl Default for Ui {
	fn default() {

	}
}

pub struct FiberUi;

impl Plugin for FiberUi {
	fn build(&self, app: &mut App) {
		app.add_plugin(FrameRate)
		.add_startup_system(ui_setup);
	}
}