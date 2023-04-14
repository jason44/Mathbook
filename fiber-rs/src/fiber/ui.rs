use std::{ops::RangeInclusive, str::FromStr, collections::LinkedList};
use bevy::{
	prelude::*, transform, render::{texture, color::Color}
};
use bevy_egui::*;
use bevy_egui::egui::{
	FontDefinitions, TextStyle, FontId, FontFamily,
	TextEdit,
};
use regex::Regex;
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

struct Image {
	image: Handle<texture::Image>,
	image_inverted: Handle<texture::Image>,
} 

impl Image {
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

#[inline]
fn remove_whitespaces(string: &mut String) -> String {
	let re = Regex::new(r"\s+").unwrap();
	let res = re.replace_all(string.as_str(), "");
	String::from_str(&res).unwrap()
}

#[derive(Resource)]
struct Functions {
	pub call:  LinkedList<Option<fn(f32) -> f32>>,
	pub re: Regex,
}
use u32 as FunctionIdx;

impl Default for Functions {
	fn default() -> Self {
		Functions {
			call: LinkedList::new(), 
			// '/' does not need to be escaped
			re: Regex::new(r"\D{3,4}?\(\w+\)|\d+|\+|\-|\*|/|\(|\)|\^").unwrap()
		}
	}
}

impl Functions {
	fn from_string(&mut self, string: String) {
	}

	fn tokenize_string(&self, string: &mut String) {
		let s = remove_whitespaces(string);
		println!("{}", s);
		let tokens: Vec<&str> = self.re.split(s.as_str()).collect();
		//for c in  string.char_indices(){
		println!("{:?}", tokens);
	}

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
	let ctx = contexts.ctx_mut();
	if !state.inverted {
		ctx.set_visuals(egui::Visuals {
			window_rounding: 10.0.into(),
			menu_rounding: 10.0.into(),
			dark_mode: true,
			window_shadow: egui::epaint::Shadow::small_dark(),
			button_frame: true,
			..Default::default()
		});	
	} else {
		ctx.set_visuals(egui::Visuals {
			window_rounding: 10.0.into(),
			menu_rounding: 10.0.into(),
			dark_mode: false,
			window_shadow: egui::epaint::Shadow::small_light(),
			button_frame: true,
			..Default::default()
		});	
	}

	// configure fonts
	let mut fonts = FontDefinitions::default();
	fonts.font_data.insert(
		"default-font".to_owned(), 
		egui::FontData::from_static(include_bytes!(
			"../../assets/fonts/Roboto-Regular.ttf"
		))
	);
	fonts.font_data.insert(
		"default-font-bold".to_owned(),
		egui::FontData::from_static(include_bytes!(
			"../../assets/fonts/Roboto-Medium.ttf"
		))
	);
	fonts.families
		.entry(egui::FontFamily::Name("regular".into()))
		.or_default()
		.insert(0, "default-font".to_owned());
	fonts.families
		.entry(egui::FontFamily::Name("bold".into()))
		.or_default()
		.insert(0, "default-font-bold".to_owned());

	ctx.set_fonts(fonts);

	// configure text styles
	let mut style = (*ctx.style()).clone();
	style.text_styles = [
		(TextStyle::Heading, FontId::new(24.0, FontFamily::Name("bold".into()))),
		(TextStyle::Body, FontId::new(11.0, FontFamily::Name("regular".into()))),
		(TextStyle::Button, FontId::new(12.0, FontFamily::Name("bold".into()))),
		(TextStyle::Small, FontId::new(9.0, FontFamily::Name("regular".into())))
	].into();
	ctx.set_style(style);				
}

fn ui_system(
	mut ui_state: ResMut<UiState>, 
	mut canvas_info: ResMut<CanvasInfo>,
	transform_info: Res<TransformInfo>,
	mut contexts: EguiContexts
) {
	let ctx = contexts.ctx_mut();
	//ctx.set_pixels_per_point(2.0);
	egui::SidePanel::left("side_panel")
		.default_width(200.0)
		.show(ctx, |ui| {
			ui.spacing_mut().item_spacing = egui::vec2(2.0, 12.0);
			
			// center along vertical axis
			ui.vertical_centered(|ui| {
				ui.heading("fiber graph");
			});

			// .horizontal positions all children next to each other horizontally
			//ui.horizontal(|ui| {});
			ui.label("input functions here");
			ui.add(TextEdit::singleline(
				&mut ui_state.func_text[0]).margin(egui::Vec2::new(4.0, 6.0)
			));

			ui.add(egui::Slider::new(
				&mut canvas_info.transform_pos, 
				RangeInclusive::new(0, transform_info.steps as u32))
				.text("transformation")
			);

			ui.label("input transformation matrices here");
			ui.add(TextEdit::singleline(
				&mut ui_state.mat_text[0]).margin(egui::Vec2::new(4.0, 6.0)
			));

			ui.add_space(5.0);
			if ui.button("apply transformation").clicked() {
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

#[cfg(test)]
mod tests {
	use crate::fiber::ui::*;
	#[test]
	fn regex_test() {
		let f = Functions::default();
		f.tokenize_string(&mut String::from_str(" tan(x)+ 5").unwrap());

	}
}
 