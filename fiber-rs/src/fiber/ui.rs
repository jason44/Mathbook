use bevy::{
	prelude::*
};

use bevy_egui::*;

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

struct UiImage {
	image: Handle<Image>,
	image_inverted: Handle<Image>,
} 

impl UiImage {
	fn from_path(asset_server: &mut AssetServer, path: String) -> Self {
		let (name, ftype) = path.split_once('.').unwrap();
		let inverted_path = String::with_capacity(name.len() + 10 + ftype.len()) + name + "-inverted." + ftype;
		Self {
			image: asset_server.load(path),
			image_inverted: asset_server.load(inverted_path),
		}	
	}
	//let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
}

#[derive(Default, Resource)]
pub struct UiState {
	// dark mode is default. inverted=true is light mode
	pub inverted: bool,
	pub is_visible: bool,	
	pub egui_texture_handle: Option<egui::TextureHandle>,
}

fn egui_style_config(mut contexts: EguiContexts, mut state: ResMut<UiState>) {
	if !state.inverted {
		contexts.ctx_mut().set_visuals(egui::Visuals {
			window_rounding: 5.0.into(),
			dark_mode: true,
			window_shadow: egui::epaint::Shadow::small_dark(),
			..Default::default()
		});	
	} else {
		contexts.ctx_mut().set_visuals(egui::Visuals {
			window_rounding: 5.0.into(),
			dark_mode: false,
			window_shadow: egui::epaint::Shadow::small_light(),
			..Default::default()
		});	
	}
}



pub struct FiberUi;

impl Plugin for FiberUi {
	fn build(&self, app: &mut App) {
		app.add_plugin(FrameRate)
		.add_plugin(EguiPlugin)
		.add_startup_system(ui_setup);
	}
}
