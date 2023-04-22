use std::{
	ops::RangeInclusive, str::FromStr, collections::{LinkedList, HashMap}, 
	borrow::BorrowMut, mem::*
};
use bevy::{
	prelude::*, transform, render::{texture, color::Color}
};
use bevy_egui::*;
use bevy_egui::egui::{
	FontDefinitions, TextStyle, FontId, FontFamily,
	TextEdit,
};
use regex::Regex;
use lazy_static::*;
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

#[repr(C)]
union TokenValue {
	c: char,
	i: i32,
}

// we want as many functions as possible to be predefined as a Tokens so 
// we do not need to rely on Token::FUNC() which is reserved for user defined functions
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Token {
	// NUM needs to be casted to a float (using mem::transmute)
	LEFT_PAREN, RIGHT_PAREN, PLUS, MINUS, 
	MUL, DIV, NUM(i32), VAR(char), FUNC(&'static str), 
	POW, SIN, COS, TAN, ASIN, ACOS, ATAN, 
	CSC, SEC, COT, SQRT, DDX, INT, INVALID
}

impl Token {
	fn value(self) -> TokenValue {
		match self {
			Token::VAR(val) => TokenValue{c: val},
			Token::NUM(val) => TokenValue{i: val},
			_ => panic!("called Token::unwrap on a Token other than VAR and NUM")
		}
	}
}

#[derive(Clone, Copy)]
enum TokenType {
	// functions need to be enclosed in parenthesis unlike operations
	// they will be handled differently
	OPERATOR(i32), OPERAND, FUNCTION(i32)
}

impl TokenType {
	fn unwrap(self) -> i32 {
		match self {
			TokenType::OPERATOR(val) => val,
			TokenType::FUNCTION(val) => val,
			_ => panic!("called TokenType::unwrap on a TokenType::OPERAND value")
		}
	}
}


lazy_static! {
	// we need &'static so that each key in the pair lives for the entire lifetime of the program
	static ref token_map: HashMap<&'static str, Token> = {
		let mut map = HashMap::new();
		map.insert("(", Token::LEFT_PAREN);
		map.insert(")", Token::RIGHT_PAREN);
		map.insert("+", Token::PLUS);
		map.insert("-", Token::MINUS);
		map.insert("*", Token::MUL);
		map.insert("/", Token::DIV);
		map.insert("^", Token::POW);
		map.insert("sin", Token::SIN);
		map.insert("cos", Token::COS);
		map.insert("tan", Token::TAN);
		map.insert("asin", Token::ASIN);
		map.insert("acos", Token::ACOS);
		map.insert("atan", Token::ATAN);
		map.insert("csc", Token::CSC);
		map.insert("sec", Token::SEC);
		map.insert("cot", Token::COT);
		map.insert("sqrt", Token::SQRT);
		map.insert("ddx", Token::DDX);
		map.insert("int", Token::INT);
		map
	};
}

lazy_static! {
	static ref type_map: HashMap<Token, TokenType> = {
		let mut map = HashMap::new();
		// operators are given precedence where the larger one is applied first
		map.insert(Token::LEFT_PAREN, TokenType::OPERATOR(-1));
		map.insert(Token::RIGHT_PAREN, TokenType::OPERATOR(-1));
		map.insert(Token::PLUS, TokenType::OPERATOR(1));
		map.insert(Token::MINUS, TokenType::OPERATOR(1));
		map.insert(Token::MUL, TokenType::OPERATOR(2));
		map.insert(Token::DIV, TokenType::OPERATOR(2));
		map.insert(Token::POW, TokenType::OPERATOR(3)); 
		map.insert(Token::SIN, TokenType::FUNCTION(0));
		map.insert(Token::COS, TokenType::FUNCTION(0));
		map.insert(Token::TAN, TokenType::FUNCTION(0));
		map.insert(Token::ASIN, TokenType::FUNCTION(0));
		map.insert(Token::ACOS, TokenType::FUNCTION(0));
		map.insert(Token::ATAN, TokenType::FUNCTION(0));
		map.insert(Token::CSC, TokenType::FUNCTION(0));
		map.insert(Token::SEC, TokenType::FUNCTION(0));
		map.insert(Token::COT, TokenType::FUNCTION(0));
		map.insert(Token::SQRT, TokenType::FUNCTION(0));
		map.insert(Token::DDX, TokenType::FUNCTION(0));
		map.insert(Token::INT, TokenType::FUNCTION(0));
		map
	};
}

#[derive(Default)]
struct Function {
	pub call: Option<fn (f32) -> f32>,
}

impl Function {
	
}

#[derive(Resource)]
struct Functions {
	pub calls:  LinkedList<Function>,
	re: Regex,
	digit_re: Regex,
	alpha_re: Regex,
}
use u32 as FunctionIdx;

impl Default for Functions {
	fn default() -> Self {
		Functions {
			calls: LinkedList::new(), 
			// '/' does not need to be escaped
			//re: Regex::new(r"(\w{3,4})\(|(\w)|(\d+)|(\+)|(\-)|(\*)|(/)|(\()|(\))|(\^)").unwrap(),
			//re: Regex::new(r"\D{3,4}\(\w+\)|\w+|\d+|\+|\-|\*|/|\(|\)|\^").unwrap(),
			re: Regex::new(r"\[a-zA-Z]{3,4}|\+|\-|\*|/|\(|\)|\^|[a-zA-Z]+|[0-9]+").unwrap(),
			digit_re: Regex::new(r"\d+").unwrap(),
			alpha_re: Regex::new(r"\D+").unwrap(),
		}
	}
}

#[inline]
fn add(x: f32, l: f32, r: f32) -> f32 {
	let mut lv = l;
	let mut rv = r;
	if lv == ONE32F {lv = x}
	if rv == ONE32F {rv = x}
	lv + rv
}

#[inline]
fn sub(x: f32, l: f32, r: f32) -> f32 {
	let mut lv = l;
	let mut rv = r;
	if lv == ONE32F {lv = x}
	if rv == ONE32F {rv = x}
	lv - rv
}

#[inline]
fn mul(x: f32, l: f32, r: f32) -> f32 {
	let mut lv = l;
	let mut rv = r;
	if lv == ONE32F {lv = x}
	if rv == ONE32F {rv = x}
	lv * rv
}

#[inline]
fn div(x: f32, l: f32, r: f32) -> f32 {
	let mut lv = l;
	let mut rv = r;
	if lv == ONE32F {lv = x}
	if rv == ONE32F {rv = x}
	lv / rv
}

#[inline]
fn pow(x: f32, l: f32, r: f32) -> f32 {
	let mut lv = l;
	let mut rv = r;
	if lv == ONE32F {lv = x}
	if rv == ONE32F {rv = x}
	lv.powf(rv)
}

const ONE32F: f32 = unsafe{transmute::<i32, f32>(0xFFFF)};
fn interpret_term(operator: Token, left: Token, right: Token) -> Box<dyn FnMut(f32) -> f32> {
	let l = left.value();
	let r = right.value();
	let mut lv;
	let mut rv;
	match l {
		TokenValue{c} => lv = ONE32F,
		TokenValue{i} => unsafe{transmute::<i32, f32>(i);}
	}
	match r {
		TokenValue{c} => rv = ONE32F,
		TokenValue{i} => rv = unsafe{transmute::<i32, f32>(i)}
	}
	match operator {
		Token::PLUS => {
			let __add = |x| -> f32 {add(x, lv, rv)};
			Box::new(__add)
		},
		Token::MINUS => {
			let __sub = |x| -> f32 {sub(x, lv, rv)};
			Box::new(__sub)
		},
		Token::MUL => {
			let __mul = |x| -> f32 {mul(x, lv, rv)};
			Box::new(__mul)
		},
		Token::DIV => {
			let __div = |x| -> f32 {div(x, lv, rv)};
			Box::new(__div)
		},
		Token::POW => {
			let __pow = |x| -> f32 {pow(x, lv, rv)};
			Box::new(__pow)
		},
		_ => panic!("The given operator is not a valid operator Token")
	}
}

impl Functions {
	fn tokenize(&self, s: &str) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::with_capacity(20);
		for cap in self.re.captures_iter(s) {
			for i in 0..cap.len() {
				//println!("var: {}", &cap[i]);
				let t = token_map.get(&cap[i].trim());
				let token: Token = match t {
					Some(e) => e.clone(),
					None => {
						let digit_arm = match self.digit_re.is_match(&cap[i]) {
							true => Token::NUM(
								// thanks Rust, I guess...
								unsafe {std::mem::transmute::<f32, i32>(cap[i].parse::<f32>().unwrap())}
							),
							_ => {Token::INVALID}
						};
						let alpha_arm = match self.alpha_re.is_match(&cap[i]) {
							true => Token::VAR(cap[i].parse::<char>()
								.expect("CANNOT CONVERT TO CHAR")),
							_ => {Token::INVALID}
						};
						if digit_arm != Token::INVALID {digit_arm}
						else {alpha_arm} 
					}
				};
				tokens.push(token);
			}
		}
		tokens
	}

	fn parse_string(&self, string: &mut String) {
		let s = remove_whitespaces(string);
		let mut tokens = self.tokenize(s.as_str());
		//let mut objects: Vec<Token> = Vec::new();
		//let mut operations: Vec<Token> = Vec::new();
		let mut operand_stack: Vec<Token> = Vec::new();	
		let mut operator_stack: Vec<Token> = Vec::new();	

		let terms: Vec<fn (f32, f32) -> f32>;

		for token in tokens {	
			let toktype = type_map.get(&token);
			match toktype {
				// operands are not mapped to type_map because they contain values
				None => operand_stack.push(token),
				Some(TokenType::OPERATOR(precedence)) => {
					// evaluate as long as an operator is already in the stack and 
					// the newest operator is not '(' because we have to evaluate expressions in paranthesis first 
					// also check precedence (everything should work by communitive property of multiplication and addition)
					while operator_stack.is_empty() == false && 
						  operator_stack[operator_stack.len()] != Token::LEFT_PAREN && 
						  precedence <= 
						  &(type_map.get(&operator_stack[operator_stack.len()])
						  .unwrap().unwrap()) {
						let right = operand_stack.pop();
						let left = operand_stack.pop();
						terms.push()
					}
					operator_stack.push(token);
				}		
				_ => {}
			}
			match token {
				Token::LEFT_PAREN => {},
				Token::RIGHT_PAREN => {}
			}
		}
	}

	fn from_string(&mut self, mut string: String) {
		self.calls.push_back(Function::default());

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
		f.parse_string(&mut String::from_str(" tan(x)+ (5+20x ) - 2x^2").unwrap());

	}
}
 