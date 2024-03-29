use std::{borrow::BorrowMut, f32::consts::PI};
use bevy::{
	prelude::*, 
	input::{
		ButtonState,keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings
};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use crate::fiber::ui::UiState;

// TODO: fix gridlines changing to n-1 whenever n-1/5
// use ndarray functions to compute samples

// prevents instantiation
#[non_exhaustive]
struct PaintStyle;

// read about associated constants for more info
impl PaintStyle {
	pub const COLOR: Color = Color::rgba(0.25, 0.55, 0.85, 0.55);
	pub const COLOR_ALT: Color = Color::rgba(0.85, 0.25, 0.30, 1.0);
	pub const COLOR_MAJOR: Color = Color::rgba(0.25, 0.55, 0.85, 1.0);
	pub const COLOR_MINOR: Color =  Color::rgba(0.1875, 0.412, 0.6375, 1.0);
	pub const THICKNESS: f32 =  0.85;
	pub const THICKNESS_MAJOR: f32 =  2.9;
	pub const THICKNESS_MINOR: f32 =  0.75;
}

type SingleVarFunc = fn(f32) -> f32;

#[derive(Resource)]
pub struct CanvasInfo {
	// width and height of the window
	width: f32,
	height: f32,
	// the scale value that all CanvasComponents will be scaled by.
	// It is calculated as the average of the window width and height
	scale: f32,
	// the bases of the Canvas coordinate system. (they are both 80)
	i: f32,
	j: f32,
	// translation is the translation on the camera (not the linear map!) 
	// divide translation by ij and cam_scale to get functional values
	translation: Vec3,
	// dtranslation is used to determine when new gridlines and cuves should be drawn
	dtranslation: Vec2,
	// drag determines whether the left mouse button is down 
	drag: bool,
	// the previous cursor position used to calculate direction the camera should move 
	prev: Vec2,
	// how far out the camera is 'zooming' out. values >1 would zooming the camera in 
	cam_scale: f32,
	// the camera scaling local to the gridlines
	cam_local_scale: f32,
	// an integer that decides when the spacing between gridlines should change
	grid_zoom_level: i32,
	// functions to be drawn
	funcs: Vec<SingleVarFunc>,
	// transforms to be played either as a composition or individually
	transforms: Vec<Mat4>,
	pub transform_pos: u32
}

impl CanvasInfo {
	fn push_func(&mut self, func: SingleVarFunc) -> &mut CanvasInfo {
		self.funcs.push(func);	
		self
	}

	fn push_transform(&mut self, transform: Mat4) -> &mut CanvasInfo {
		self.transforms.push(transform);	
		self
	}
}

impl Default for CanvasInfo {
	fn default() -> Self {
		CanvasInfo {
			width: 1280.0, height: 720.0, scale: 1.0, 
			translation: Vec3::ZERO, dtranslation: Vec2::ZERO,
			i: 80.0, j: 80.0, prev: Vec2::ZERO, drag: false, 
			cam_scale: 1.0, cam_local_scale: 1.0, grid_zoom_level: 0, 
			funcs: Vec::new(), transforms: Vec::new(), transform_pos: 0
		}
	}
}

#[derive(Resource)]
pub struct TransformInfo {
	pub base: Mat4,
	pub target: Mat4,
	pub delta: Mat4,
	pub steps: f32,
}

impl TransformInfo {
	fn sub_delta(&mut self) {
		self.base -= self.delta;
	}

	fn add_delta(&mut self) {
		self.base += self.delta;
	}
}

impl Default for TransformInfo {
	fn default() -> TransformInfo {
		TransformInfo {
			base: Mat4::ZERO,
			target: Mat4::ZERO,
			delta: Mat4::ZERO,
			steps: 0.0,
		}
	}
}

#[derive(Component)]
struct CanvasComponent;

pub struct Canvas;

impl Plugin for Canvas {
	fn build(&self, app: &mut App) {
		app.insert_resource(CanvasInfo::default())
		.insert_resource(TransformInfo::default())
		.add_startup_system(grid_startup)
		.add_system(resize_system)
		.add_system(mouse_system)
		.add_system(keybind_system);
	}
}

fn draw_grid(_commands: &mut Commands, canvas_info: &CanvasInfo) {
	let x_incr = canvas_info.i * canvas_info.cam_local_scale; 
	let y_incr = canvas_info.j * canvas_info.cam_local_scale;
	// draw lines that are out of view so that we do not have to redraw as often
	let l = canvas_info.width * 3.0 * canvas_info.cam_local_scale;
	let h = canvas_info.height * 3.0 * canvas_info.cam_local_scale;
	let xf = (canvas_info.translation.x / x_incr).floor() * x_incr;
	let yf = (canvas_info.translation.y / y_incr).floor() * y_incr;

	let mut builder = GeometryBuilder::new();
	let mut i: f32 = 0.0;
	while (i*x_incr) < l || (i*y_incr) < h {
		builder = builder
		.add(&Line(
			Vec2::new(xf + (i*x_incr), yf-h),
			Vec2::new(xf + (i*x_incr), yf+h)
		))
		.add(&Line(
			Vec2::new(xf - (i*x_incr), yf-h),
			Vec2::new(xf - (i*x_incr), yf+h)
		))
		.add(&Line(
			Vec2::new(xf-l, yf + (i*y_incr)),
			Vec2::new(xf+l, yf + (i*y_incr))
		))
		.add(&Line(
			Vec2::new(xf-l, yf - (i*y_incr)),
			Vec2::new(xf+l, yf - (i*y_incr))
		)); 
		i += 1.0; 
	}

	_commands.spawn((
		ShapeBundle {
			path: builder.build(),
			transform: Transform::from_scale(Vec3::new(
				canvas_info.scale, canvas_info.scale, 1.0
			)),
			..default()
		},
		//Fill::color(PaintStyle::COLOR),
		Stroke::new(PaintStyle::COLOR_MINOR, 
			PaintStyle::THICKNESS_MINOR*canvas_info.cam_scale
		),
		CanvasComponent,
	));

	let axes_builder = GeometryBuilder::new()
	.add(&Line(
		Vec2::new(0.0, -h - canvas_info.translation.y.abs()),
		Vec2::new(0.0, h + canvas_info.translation.y.abs())
	)).add(&Line(
		Vec2::new(-l - canvas_info.translation.x.abs(), 0.0),
		Vec2::new(l + canvas_info.translation.x.abs(), 0.0)
	));

	_commands.spawn((
		ShapeBundle {
			path: axes_builder.build(),
			..default()
		},
		//Fill::color(PaintStyle::COLOR_MAJOR),
		Stroke::new(PaintStyle::COLOR_MAJOR, 
			PaintStyle::THICKNESS_MAJOR*canvas_info.cam_scale
		),
		CanvasComponent,
	));
}

/// @param x_range
/// the range in which the function is drawn (in functional coordinates, not screen coordinates) 
/// @param resolution:
/// the number of points to be sampled
fn draw_function<F>(
	_commands: &mut Commands, 
	canvas_info: &CanvasInfo, 
	f: F, 
	x_range: (f32, f32), 
	resolution: f32)
	where F: Fn(f32) -> f32 
{
	let nx = resolution as usize;
	let dx = (x_range.1 - x_range.0) / resolution;
	let l = x_range.0;
	
	let mut t: f32 = 0.0;
	let mut n = 0;
	let mut points: Vec<Vec2> = Vec::with_capacity(nx);
	while l + (t * dx) < x_range.1 {
		// possible optimizations:
		// end the loop earlier instead of incrementing n to nx (especially helpful for a function that is only defined for non-negative numbers)
		// bug: functions with slopes that increase too quickly are not adequately graphed. 

		// possible solution?: scale dx based on the derivative of the current point  
		// if that doesn't work well enough, we can just draw a vertical line if the derivative is large enough. 
		// (maybe a combination of both)
		// this will fix bugs with functions like tanx and sqrtx
		for _i in n..nx {
			let x = (l + (t * dx)) * canvas_info.i;
			let y = f(l + (t * dx)) * canvas_info.j;
			t += 1.0;
			if y.is_finite() == false {break}
			points.push(Vec2::new(x, y))
		}
		n += 1;

		if n == nx-1 {break}
		if points.len() == 0 {continue}
		let mut path_builder = PathBuilder::new();
		path_builder.move_to(points[0]);	
		for i in 1..points.len() {path_builder.line_to(points[i]);}
		//path_builder.close();

		_commands.spawn((
			ShapeBundle {
				path: path_builder.build(),
				transform: Transform::from_scale(Vec3::new(
					canvas_info.scale, canvas_info.scale, 1.0
				)),
				..default()
			},
			Stroke::new(PaintStyle::COLOR_ALT, 
				PaintStyle::THICKNESS_MAJOR*canvas_info.cam_scale
			),
			//Fill::color(PaintStyle::COLOR_MINOR),
			CanvasComponent
		));
		points.clear();
	}
}

fn draw(_commands: &mut Commands, canvas_info: &CanvasInfo) {

	draw_grid(_commands, canvas_info);
	let x_range = (
		// width is multiplied by 3 so that the function is drawn for 1 whole width to the left and right of the viewport
		// which is consistent with what we do when calculating gridlines
		// TODO: gridlines and functions use the same x_range, so instead of calculating it twice, calculate it once here
		((canvas_info.translation.x - (canvas_info.width * canvas_info.cam_scale * 3.0)) / canvas_info.i),
		((canvas_info.translation.x + (canvas_info.width * canvas_info.cam_scale * 3.0)) / canvas_info.i)
	);
	let res_scale = |x: f32| {(2.0/x)+0.2};
	println!("width: {}", canvas_info.width);
	println!("x_range ({},{})", x_range.0, x_range.1);
	println!("resolution {}", 40.0 * res_scale(canvas_info.cam_scale) * (x_range.1-x_range.0));
	//println!("resolution {}", (canvas_info.height + canvas_info.width) * canvas_info.cam_scale * res_scale(canvas_info.cam_scale));
	for func in &canvas_info.funcs {
		draw_function(_commands, canvas_info, func, x_range, 
			40.0 * res_scale(canvas_info.cam_scale) * (x_range.1-x_range.0));
			//(canvas_info.height + canvas_info.width) * canvas_info.cam_scale * res_scale(canvas_info.cam_scale));
	}
}

fn grid_startup(mut _commands: Commands, mut canvas_info: ResMut<CanvasInfo>) {
	canvas_info
	.push_func(|x|{x.exp()})
	//.push_func(|x|{x*0.5})
	//.push_func(|x|{x.sqrt()})
	.push_func(|x|{x.ln()})
	.push_func(|x|{1.0 / x})
	.push_func(|x|{x.sin()});
	//.push_func(|x|{x})
	//.push_func(|x|{(x+2.0).sqrt()+x.powf(2.0)})
	//.push_func(|x|{x.tan()});

	draw(&mut _commands, &canvas_info);
}

fn resize_system(mut _commands: Commands,
	mut canvas_info: ResMut<CanvasInfo>,
	mut transform: Query<&mut Transform, With<CanvasComponent>>,
	mut resize_reader: EventReader<WindowResized>
) {
	for event in resize_reader.iter() {
		let y = event.height;
		let x = event.width;

		let scalex = x / canvas_info.width;
		let scaley = y / canvas_info.height;
		let scale = (scalex + scaley) / 2.0;
		for mut transform in transform.iter_mut() {
			transform.scale.x *= scale;
			transform.scale.y *= scale;
		}
		canvas_info.scale *= scale;
		canvas_info.height = y;
		canvas_info.width = x;
		//draw_grid(&mut _commands, &canvas_info);
	}
	resize_reader.clear();
}

fn mouse_system(
	mut _commands: Commands,
	mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
	//mut transforms: Query<&mut Transform, (With<CanvasComponent>, Without<Camera>)>,
	mut cam_transforms: Query<&mut Transform, With<Camera>>,
	old_elements: Query<Entity, With<CanvasComponent>>,
	mut canvas_info: ResMut<CanvasInfo>
) {

	for event in mouse_button_input_events.iter() {
		if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
			canvas_info.drag = true;
			break;
		}
		if event.button == MouseButton::Left && event.state == ButtonState::Released {
			canvas_info.drag = false;
			break;
		}
    }

	for cursor in cursor_moved_events.iter() {
		if canvas_info.drag {
			for mut transform in cam_transforms.iter_mut() {
				let dir = canvas_info.prev - cursor.position;
				let dx = dir.x * 0.75 * canvas_info.cam_scale;
				let dy = dir.y * 0.75 * canvas_info.cam_scale;
				transform.translation.x += dx;
				transform.translation.y += dy;
				canvas_info.dtranslation.x += dx;
				canvas_info.dtranslation.y += dy;
				canvas_info.translation.x += dx;
				canvas_info.translation.y += dy;
			}

			// there are 16 horizontal squares visible at 1.0 scale and 9 vertical squares. 
			// we redraw a bit earlier (every 8 squares) so that we can hide any visual inconsistencies caused by delays
			if canvas_info.dtranslation.x.abs() > canvas_info.i * 8.0 * canvas_info.cam_scale || 
				canvas_info.dtranslation.y.abs() > canvas_info.j * 8.0 *  canvas_info.cam_scale {
				draw(&mut _commands, &canvas_info);
				println!("xtranslation: {}, ytranslation: {}", canvas_info.translation.x, canvas_info.translation.y);
				canvas_info.dtranslation = Vec2::ZERO;
			} 
		}
		canvas_info.prev = cursor.position;
	}

    for event in mouse_wheel_events.iter() {
		if event.y > 0.0 {canvas_info.cam_scale *= 0.8; canvas_info.grid_zoom_level -= 1;} 
		else {canvas_info.cam_scale *= 1.2; canvas_info.grid_zoom_level += 1}

		if canvas_info.grid_zoom_level % 5 == 0 {canvas_info.cam_local_scale = canvas_info.cam_scale;}

		for element in old_elements.iter() {_commands.entity(element).despawn()} 
		for mut transform in cam_transforms.iter_mut() {
			transform.scale.x = canvas_info.cam_scale;
			transform.scale.y = canvas_info.cam_scale;
			//canvas_info.scale *= canvas_info.cam_scale;
			draw(&mut _commands, &canvas_info);
		}
		println!("SWITCH: {}", canvas_info.grid_zoom_level);
    }
	// to get l (horizontal distance from origin) out of translation.x: translation.x = l*i*cam_scale. 
	// So, to get horizontal distance in functional coordinates just use the formula:
	// translation.x/i*cam_scale afterwards, shift the result by translation.x and change bases to ij.
	mouse_button_input_events.clear();
	mouse_wheel_events.clear();
	cursor_moved_events.clear();
}

fn keybind_system(
	mut _commands: Commands, 
	mut transforms: Query<&mut Transform, With<CanvasComponent>>,
	mut canvas_info: ResMut<CanvasInfo>,
	transform_info: ResMut<TransformInfo>,
	old_elements: Query<Entity, With<CanvasComponent>>,
	key: Res<Input<KeyCode>>,
	//time: Res<Time>
) {
	let mut dx: f32 = 0.0;
	let mut dy: f32 = 0.0;
	let mut ti = transform_info;
	if key.just_pressed(KeyCode::A) {
		canvas_info.push_transform(Mat4::from_cols(
			Vec4::new((PI/2.0).cos(), -(PI/2.0).sin(), 0.0, 0.0),
			Vec4::new((PI/2.0).sin(), (PI/2.0).cos(), 0.0, 0.0),
			Vec4::new(0.0, 0.0, 1.0, 0.0),
			Vec4::new(0.0, 0.0, 0.0, 1.0),
		));
	}

	if key.just_pressed(KeyCode::Space) {
		let mut comp: Mat4;
		canvas_info.transforms.reverse();
		let mut iter = canvas_info.transforms.iter();
		comp = iter.next().unwrap_or(
			/*
			&Mat4::from_diagonal(
				Vec4::new(1.0, 2.0, 1.0, 1.0)
			) */
			&Mat4::from_cols(
				Vec4::new(3.0, -(PI/2.0).sin(), 0.0, 0.0),
				Vec4::new(2.0, -4.0, 0.0, 0.0),
				Vec4::new(0.0, 0.0, 1.0, 0.0),
				Vec4::new(0.0, 0.0, 0.0, 1.0),
			)
		).clone();
		for mat in iter {
			comp.mul_mat4(&mat);
		}
		println!("target: {}", comp);

		let t = transforms.iter().next().unwrap();
		ti.base = t.compute_matrix();
		println!("ti.base: {}", ti.base);
		//ti.target = comp.mul_mat4(&ti.base);
		ti.target = comp;
		ti.delta = ti.target - ti.base;
		println!("ti.delta: {}", ti.delta);
		ti.steps = (50.0 * (
			(ti.delta.x_axis.x.abs() + ti.delta.x_axis.y.abs() + 
			ti.delta.y_axis.x.abs() + ti.delta.y_axis.y.abs()) / 4.0
		)).ceil();
		ti.delta = ti.delta.mul_scalar(1.0 / ti.steps);
		println!("ti.delta real: {}", ti.delta);
		println!("steps: {}", ti.steps);
    }
    if key.pressed(KeyCode::Left) {
		if canvas_info.transform_pos > 0 {
			// TODO: make the base transformation apply to all new drawings
			// (but for simplicity, reset the transformation when the cameara moves too far)
			// (otherwise, we must modify xf and yf based on the rotation applied by the matrix)
			
			// suggestion: redraw grid every couple increments with 
			// line thickness scaled by the the average of x, y in the composition's columns 
			// [x, c], [c, y]
			println!("bases: {:?}", ti.base);
			println!("target {:?}", ti.target);
			println!("delta: {:?}", ti.delta);
			println!("------------------");
			canvas_info.transform_pos -= 1;
			ti.sub_delta();
			for mut transform in transforms.iter_mut() {
				//ti.base -= ti.delta;
				*transform = Transform::from_matrix(ti.base.clone());
				
			}
		}
    }

    if key.pressed(KeyCode::Right) {
		println!("steps: {}", ti.steps);
		if canvas_info.transform_pos < ti.steps as u32 {
			println!("bases: {:?}", ti.base);
			println!("target {:?}", ti.target);
			println!("delta: {:?}", ti.delta);
			println!("------------------");
			canvas_info.transform_pos += 1;
			ti.add_delta();
			for mut transform in transforms.iter_mut() {
				//ti.base += ti.delta;
				*transform = Transform::from_matrix(ti.base.clone());
			}
			// while loop isn't called ever
		}
    }

    if key.just_released(KeyCode::P) {
		for entity in old_elements.iter() {
			_commands.entity(entity).despawn();
		}
		draw_grid(&mut _commands, &canvas_info);
        info!("'A' just released");
    }
	if key.pressed(KeyCode::P) {
		dx -= 0.05;	
		dy -= 0.02;	
		let m = Mat4::from_diagonal(Vec4::new(
			dx, dy, 1.0, 1.0
		));
		for mut transform in transforms.iter_mut() {
			let a = transform.compute_matrix();
			println!("a: {}", a);
			*transform = Transform::from_matrix(a.mul_mat4(&m));
		}
	}
}
