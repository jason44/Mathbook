use std::{borrow::BorrowMut, f32::consts::PI};
use bevy::{
	prelude::*, 
	input::{
		ButtonState, keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings,
	render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin}
};
use bevy_prototype_lyon::{prelude::*, shapes::*};

mod fiber;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
		//.insert_resource(ClearColor(Color::hex("#f5e5ba").unwrap()))
		// only run the app when there is user input; reduce CPU/GPU usage
		//.insert_resource(WinitSettings::desktop_app()) 		
		.add_plugins(DefaultPlugins.set(RenderPlugin {
			wgpu_settings: WgpuSettings {
				features: WgpuFeatures::POLYGON_MODE_LINE,
				..default()
			}
		}))
		.add_plugin(ShapePlugin)
		//.add_plugin(fiber::ui::FiberUi)
		//.add_plugin(fiber::canvas::Canvas)
		.add_plugin(fiber::canvas3d::Canvas3D)
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
	//_commands.spawn(Camera2dBundle::default());
}
