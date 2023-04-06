use bevy::{
	prelude::*, 
	input::{
		ButtonState,
		mouse::{MouseButtonInput, MouseWheel}
	},
	window::PrimaryWindow, window::WindowResized,
};

use bevy_prototype_lyon::{prelude::*, shapes::*};

mod fiber;


fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.add_plugins(DefaultPlugins)
		.add_plugin(ShapePlugin)
		.add_plugin(fiber::ui::Ui)
		.add_plugin(Gridlines)
		.add_startup_system(setup)
		//.add_system(mouse_system)
		.run();
}

fn setup(mut _commands: Commands, mut windows: Query<&mut Window>) {
	let mut window = windows.single_mut();
	window.resize_constraints = WindowResizeConstraints {
		min_height: 720.0,
		min_width: 1280.0,
		..default()
	};
}

#[derive(Component)]
struct VerticalLine;

#[derive(Component)]
struct HorizontalLine;

#[derive(Component)]
struct MajorLine;

#[derive(Component)]
struct MinorLine;

#[derive(Component)]
struct CanvasComponent;

#[derive(Resource)]
struct CanvasInfo {
	width: f32,
	height: f32,
	i: f32,
	j: f32,
	global_scale: f32,
	drag: bool,
	prev: Vec2,
	switch: bool,
}

struct Gridlines;

// prevents instantiation
#[non_exhaustive]
struct GridlineStyle;

// read about associated constants for more info
impl GridlineStyle {
	pub const COLOR: Color = Color::rgba(0.25, 0.55, 0.85, 0.55);
	pub const COLOR_MAJOR: Color = Color::rgba(0.25, 0.55, 0.85, 1.0);
	pub const COLOR_MINOR: Color =  Color::rgba(0.25, 0.55, 0.85, 0.5);
	pub const THICKNESS: f32 =  0.9;
	pub const THICKNESS_MAJOR: f32 =  2.3;
	pub const THICKNESS_MINOR: f32 =  0.65;
}

/* TODO:
 * 1. create a global resource sprite_scale: f32 which can be modified to scale all sprites drawn.
 *    this allows us to simulate zooming in and out
 * 2. approximate any given curve with lines to a certain resolution?
 * 3. create a line function since it would be a waste of compute to approximate a line with lines
 * 4. just use bevy_prototype_lyon
 * 5. transformations are done by querying for a Transform so make sure every part of the canvas has a identifier
 * 6. curves can initially be drawn in ijk and transformed to the current bases given by [x_incr, y_incr]
 * 		before drawing them. NOTE: [x_incr and y_incr] are parallel to i and j respectively
 * 6a. All other linear transformations can follow from this
 */

fn draw_grid(_commands: &mut Commands, canvas_info: &CanvasInfo) {
	let x_incr = canvas_info.i * canvas_info.global_scale;
	let y_incr = canvas_info.j * canvas_info.global_scale;
	// we add an additional 12 lines so we can keep "extend" the grid without redrawing 
	// when the window is resized.
	let x = canvas_info.width + (x_incr*12.0);
	let y = canvas_info.height + (y_incr*12.0);
	let l = 1.0*x / 2.0;
	let h = 1.0*y / 2.0;

	println!("i = {x_incr}, j = {y_incr}");
	let mut builder = GeometryBuilder::new();
	let mut i: f32 = 0.0;
	while (i*x_incr) < l || (i*y_incr) < h {
		builder = builder.add(&Line(
			Vec2::new((i*x_incr), -y),
			Vec2::new((i*x_incr), y)
		)).add(&Line(
			Vec2::new((i*x_incr)*-1.0, -y),
			Vec2::new((i*x_incr)*-1.0, y)
		));	
		builder = builder.add(&Line(
			Vec2::new(-x, (i*y_incr)),
			Vec2::new(x, (i*y_incr))
		)).add(&Line(
			Vec2::new(-x, (i*y_incr)*-1.0),
			Vec2::new(x, (i*y_incr)*-1.0)
		));
		i += 1.0;
	}

	_commands.spawn((
		ShapeBundle {
			path: builder.build(),
			..default()
		},
		Fill::color(GridlineStyle::COLOR),
		Stroke::new(GridlineStyle::COLOR, GridlineStyle::THICKNESS),
		CanvasComponent,
	));

	let axes_builder = GeometryBuilder::new()
	.add(&Line(
		Vec2::new(0.0, -y),
		Vec2::new(0.0, y)
	)).add(&Line(
		Vec2::new(-x, 0.0),
		Vec2::new(x, 0.0)
	));

	_commands.spawn((
		ShapeBundle {
			path: axes_builder.build(),
			..default()
		},
		Fill::color(GridlineStyle::COLOR_MAJOR),
		Stroke::new(GridlineStyle::COLOR_MAJOR, GridlineStyle::THICKNESS_MAJOR),
		CanvasComponent,
	));
}

fn grid_startup(mut _commands: Commands, canvas_info: Res<CanvasInfo>) {
	draw_grid(&mut _commands, &canvas_info);
}

fn grid_system(mut _commands: Commands,
	mut canvas_res: ResMut<CanvasInfo>,
	mut transform: Query<&mut Transform, With<CanvasComponent>>,
	mut resize_reader: EventReader<WindowResized>
) {
	for event in resize_reader.iter() {
		let y = event.height;
		let x = event.width;

		let scalex = x / canvas_res.width;
		let scaley = y / canvas_res.height;
		let scale = (scalex + scaley) / 2.0;
		for mut transform in transform.iter_mut() {
			transform.scale.x *= scale;
			transform.scale.y *= scale;
		}

		canvas_res.height = y;
		canvas_res.width = x;
		//draw_grid(&mut _commands, &canvas_res);
	}
}

impl Plugin for Gridlines {
	fn build(&self, app: &mut App) {
		app.insert_resource(CanvasInfo {
			width: 1280.0, height: 720.0, 
			i: 1280.0 / 16.0, j: 720.0 / 9.0, prev: Vec2::ZERO,
			drag: false, switch: false, global_scale: 1.0
		})
		.add_startup_system(grid_startup)
		.add_system(grid_system)
		.add_system(mouse_system);
	}
}

fn mouse_system(
	mut _commands: Commands,
	mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
	mut transforms: Query<&mut Transform, With<CanvasComponent>>,
	old_elements: Query<Entity, With<CanvasComponent>>,
	time: Res<Time>,
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
			for mut transform in transforms.iter_mut() {
				let dir = cursor.position - canvas_info.prev;
				// store the translation somewhere so that we can reapply it after a redraw
				transform.translation.x += dir.x * 0.6;
				transform.translation.y += dir.y * 0.6;
			}
		}
		canvas_info.prev = cursor.position;
	}

    for event in mouse_wheel_events.iter() {
		for element in old_elements.iter() {
			_commands.entity(element).despawn()
			// _commands.entity(grid).remove<CanvasComponent>();  <- removes the CanvasComponent from grid
		} 
		/* 
		let scalex = x / canvas_res.width;
		let scaley = y / canvas_res.height;
		for mut transform in transform.iter_mut() {
			let scale: f32;
			//transform.scale.x *= scaley;
			//transform.scale.y *= scaley;
		} */
		// the initial ratio is 1 ie: i=80, j = 80. To keep the grid consistent, we must perserve 
		// the ratio between i and j even when the resolution changes
		// it is more accurate to say that i and j is actually 80.0 * global_scale
		if event.y > 0.0 {canvas_info.global_scale *= 0.8;} 
		else if event.y < 0.0 {canvas_info.global_scale *= 1.2;}
		draw_grid(&mut _commands, &canvas_info);
    }
}