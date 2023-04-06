use bevy::{
	diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
	prelude::*
};

pub struct FrameRate;

#[derive(Component)]
struct FpsText;

fn framerate_text(mut _commands: Commands, asset_server: Res<AssetServer>) {
	// fps counter
	_commands.spawn((
		TextBundle::from_sections([
			TextSection::new(
				"fps: ",
				TextStyle {
					font: asset_server.load("fonts/FiraMono-Medium.ttf"),
					font_size: 20.0,
					color: Color::WHITE,
				},
			),
			TextSection::from_style(TextStyle {
				font: asset_server.load("fonts/FiraMono-Medium.ttf"),
				font_size: 20.0,
				color: Color::WHITE,
			}),
		]),
		// help identify the Text component related to the fps 
		FpsText
	));
}

fn text_update_system(
	diagnostics: Res<Diagnostics>, 
	mut query: Query<&mut Text, With <FpsText>>
) {
	for mut text in &mut query {
		if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
			if let Some(value) = fps.smoothed() {
				text.sections[1].value = format!("{value:.2}");
			}
		}
	}
}

impl Plugin for FrameRate {
	fn build(&self, app: &mut App) {
		app.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.add_startup_system(framerate_text)
		.add_system(text_update_system);
	}
}
		
