use std::collections::btree_map::Range;
use std::ops::RangeInclusive;

use bevy::{
	prelude::*, transform
};

use bevy_egui::*;
use bevy::render::color::Color;
use crate::fiber::framerate::FrameRate;
use crate::fiber::canvas::*;

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

#[derive(Resource)]
pub struct UiState {
	// dark mode is default. inverted=true is light mode
	pub inverted: bool,
	pub is_visible: bool,	
	pub egui_texture_handle: Option<egui::TextureHandle>,
	pub func_text: Vec<String>,
	pub mat_text: Vec<String>,
}

impl Default for UiState {
	fn default() -> Self {
		UiState {
			inverted: false,
			is_visible: true,
			egui_texture_handle: None,
			func_text: Vec::with_capacity(10),
			mat_text: Vec::with_capacity(10),
		}
	}
}

pub const TEXT_BUFFER_SIZE: usize = 20;
impl UiState {
	fn push_function(&mut self) {
		// 20 is a reasonable buffer 
		self.func_text.push(String::with_capacity(TEXT_BUFFER_SIZE));
	}
	fn push_transformation(&mut self) {
		// 20 is a reasonable buffer 
		self.mat_text.push(String::with_capacity(TEXT_BUFFER_SIZE));
	}
}

fn ui_startup(mut ui_state: ResMut<UiState>) {
	ui_state.push_function();
	ui_state.push_transformation();
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

fn ui_system(
	mut ui_state: ResMut<UiState>, 
	mut canvas_info: ResMut<CanvasInfo>,
	transform_info: Res<TransformInfo>,
	mut contexts: EguiContexts
) {
	let ctx = contexts.ctx_mut();
	egui::SidePanel::left("side_panel")
		.default_width(200.0)
		.show(ctx, |ui| {
			ui.heading("Fiber");
			// .horizontal positions all children next to each other horizontally
			//ui.horizontal(|ui| {
			ui.label("input functions here");
			ui.text_edit_singleline(&mut ui_state.func_text[0]);
			//});

			ui.add(egui::Slider::new(
				&mut canvas_info.transform_pos, 
				RangeInclusive::new(0, transform_info.steps as u32))
				.text("transformation")
			);

			//ui.horizontal(|ui| { 
				ui.label("input transformation matrices here");
				ui.text_edit_singleline(&mut ui_state.mat_text[0]);
			//});
			if ui.button("Apply Transformation").clicked() {
				println!("create comp");
			}
		});
	
}

pub struct FiberUi;

impl Plugin for FiberUi {
	fn build(&self, app: &mut App) {
		app.insert_resource(UiState::default())
		.add_plugin(FrameRate)
		.add_plugin(EguiPlugin)
		.add_startup_system(ui_startup)
		.add_startup_system(egui_style_config)
		.add_system(ui_system);
		//.add_startup_system(ui_setup);
	}
}
