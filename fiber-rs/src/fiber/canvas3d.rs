use std::{borrow::BorrowMut, f32::consts::PI, ops::IndexMut};
use bevy::{
	prelude::*, 
	input::{
		ButtonState, keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel, MouseMotion}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings,
	render::{mesh::{VertexAttributeValues, PrimitiveTopology, Indices}, view::NoFrustumCulling,
			 render_resource::Face},
	math::Vec3A,
	pbr::wireframe::{Wireframe, WireframePlugin}
};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use ndarray::{prelude::*, Zip};
use crate::fiber::ui::UiState;
use std::iter::FromIterator;

#[derive(Resource)]
struct Canvas3DInfo {
	// separate solids and surfaces because they 
	// are drawn with different frag shaders
	//surfaces: Vec<Handle<Mesh>>,
	//solids: Vec<Handle<Mesh>>,
	// width and height of the window
	pub width: f32,
	pub height: f32,
	pub wireframe: bool,
}

impl Default for Canvas3DInfo {
	fn default() -> Self {
		Canvas3DInfo {
			width: 1268.0,
			height: 720.0,
			wireframe: false,
		}
	}
}

#[derive(Component)]
struct SurfaceComponent;

pub struct Surface {
	xs: Array<f32, Ix1>,
	ys: Array<f32, Ix1>,
	zs: Array<f32, Ix2>,
	pub vertices: Vec<Vec3A>,
	pub indices: Array<u32, Ix1>,
	pub normals: Vec<Vec3A>,  
	pub uv_coords: Vec::<Vec2>,
	res: u32,
	func: fn(&f32, &&f32) -> f32,
}

impl Default for Surface {
	fn default() -> Self {
		Surface {
			xs: Array::<f32, _>::default(1),
			ys: Array::<f32, _>::default(1),
			zs: Array::<f32, _>::default((0, 0)),
			vertices: Vec::<Vec3A>::default(),
			indices: Array::<u32, _>::default(1),
			normals: Vec::<Vec3A>::default(),
			uv_coords: Vec::<Vec2>::default(),
			res: 101,
			func: |x, y| {x.sin() + (*y).cos()},
		}
	}
}

impl Surface {
	fn compute_points(mut self, x_range: (f32, f32), y_range: (f32, f32), res: usize) -> Self {
		let x_samples: Array1<f32> = 
			ArrayBase::linspace(x_range.0, x_range.1, res);
		let y_samples: Array1<f32> = 
			ArrayBase::linspace(y_range.0, y_range.1, res);		
		//self.points.reserve(res);
		let mut z_samples = Array::default((0, res));
		for _x in x_samples.slice(s![..]) {
			//zs.append(Axis(0), Zip::from(&arr0(_x)).and_broadcast(&y_samples).map_collect(self.func).view());
			z_samples.push_row(
				Zip::from(&y_samples)
				.and_broadcast(&arr0(_x))
				.map_collect(self.func)
				.view()
			).unwrap();
		}

		self.vertices = Vec::with_capacity(z_samples.len());
		for i in 0..res {
			for j in 0..res {
				self.vertices.push(Vec3A::new(x_samples[i], z_samples[(i, j)], y_samples[j]));
				//self.vertices.push(Vec3A::new(x_samples[i], y_samples[j], z_samples[(i, j)]));
			}
		}
		
		self.xs = x_samples;
		self.ys = y_samples;
		self.zs = z_samples;
		self.res = res as u32;
		//println!("VERTICES\n{:?}\n-----------------------------------------", self.vertices);
		self
	}

	fn compute_indices(mut self) -> Self {
		let nu = self.res as usize;
		let nv: usize = self.res as usize;
		/*let _grid = 
			Array::range(0.0, (nu * nv) as f32, 1.0);
		let grid = _grid.to_shape((nu, nv)).unwrap();*/
		let _grid = Array::from_iter(0..(nu * nv) as u32);
		let grid = _grid.to_shape((nu, nv)).unwrap();
		//let grid = __grid.t();
		let mut indices = Array1::<u32>::zeros(6 * (nu - 1) * (nv - 1));

		let top_left = Array::from_iter(grid.slice(s![..-1, ..-1]).iter().cloned());
		let bottom_left = Array::from_iter(grid.slice(s![1.., ..-1]).iter().cloned());
		let top_right = Array::from_iter(grid.slice(s![..-1, 1..]).iter().cloned());
		let bottom_right = Array::from_iter(grid.slice(s![1.., 1..]).iter().cloned());

		{let mut r = indices.slice_mut(s![0..;6]);
		for i in 0..r.len() {r[i] = top_left[i];}}
		{let mut r = indices.slice_mut(s![1..;6]);
		for i in 0..r.len() {r[i] = bottom_left[i];}}
		{let mut r = indices.slice_mut(s![2..;6]);
		for i in 0..r.len() {r[i] = top_right[i];}}
		{let mut r = indices.slice_mut(s![3..;6]);
		for i in 0..r.len() {r[i] = top_right[i];}}
		{let mut r = indices.slice_mut(s![4..;6]);
		for i in 0..r.len() {r[i] = bottom_left[i];}}
		{let mut r = indices.slice_mut(s![5..;6]);
		for i in 0..r.len() {r[i] = bottom_right[i];}}	

		//println!("INDICES\n{:?}\n---------------------------------", indices);
		self.indices = indices;	
		self
	}

	fn compute_surface_normals(mut self) -> Self {
		// unit normals
		let nu = self.res;
		let nv = self.res;
		let nunv = (nu * nv) as usize;
		let indices = Array::from_iter(0..(nu * nv));
		let mut left = vec![0u32; nunv];
		left[1..nunv].copy_from_slice(indices.slice(s![..nunv-1]).as_slice().unwrap());
		let mut right = vec![0u32; nunv];
		right[..nunv-1].copy_from_slice(indices.slice(s![1..nunv]).as_slice().unwrap());
		right[nunv-1] = indices[nunv-1];
		let mut up = vec![0u32; nunv];
		up[..nu as usize].copy_from_slice(indices.slice(s![..nu as usize]).as_slice().unwrap());
		up[nu as usize..nunv].copy_from_slice(indices.slice(s![..nunv-nu as usize]).as_slice().unwrap());
		let mut down = vec![0u32; nunv];
		down[nunv-nv as usize..].copy_from_slice(indices.slice(s![nunv-nv as usize..]).as_slice().unwrap());
		down[..nunv-nv as usize].copy_from_slice(indices.slice(s![nv as usize..nunv]).as_slice().unwrap());

		/*println!("LEFT\n{:?}\n--------------------", left);
		println!("RIGHT\n{:?}\n--------------------", right);
		println!("UP\n{:?}\n--------------------", up);
		println!("DOWN\n{:?}\n--------------------", down); */
		let vertices = &self.vertices;
		let mut crosses = Vec::<Vec3A>::with_capacity(nunv);
		for i in 0..nunv {
			/*println!("right: {}, left: {}", vertices[right[i] as usize], vertices[left[i] as usize]);
			println!("up: {}, down: {}", vertices[up[i] as usize], vertices[down[i] as usize]);
			println!("cross: {}\n-----------------\n", 
			(vertices[right[i] as usize] - vertices[left[i] as usize])
			.cross(vertices[up[i] as usize] - vertices[down[i] as usize])); */

			//crosses.push((vertices[right[i] as usize] - vertices[left[i] as usize])
			//.cross(vertices[up[i] as usize] - vertices[down[i] as usize]));

			crosses.push((vertices[up[i] as usize] - vertices[down[i] as usize])
			.cross(vertices[right[i] as usize] - vertices[left[i] as usize]));

		}
		//println!("CROSSES\n{:?}\n-------------------------", crosses);
		//self.normals = crosses;
		for normal in crosses {
			self.normals.push(normal.normalize());
		}		
		self
	}

	fn compute_uv_coords(mut self) -> Self {
		let nu = self.res as usize;
		let nv = self.res as usize;
		self.uv_coords = Vec::<Vec2>::with_capacity(nu);
		let us = Array::<f32, _>::linspace(0., 1., nu*nv);
		let vs = Array::<f32, _>::linspace(0., 1., nv*nv);
		//println!("SIZEOF us: {}", us.len());
		for i in 0..(nu*nv) {
			self.uv_coords.push(Vec2::new(us[i], vs[i]));
		}
		self
	}
}

fn canvas3d_startup(mut _commands: Commands, 
	mut meshes: ResMut<Assets<Mesh>>, 
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>
) {
	let surf = Surface::default()
	.compute_points((-20.0, 20.0), (-20.0, 20.0), 202)
	.compute_indices()
	.compute_surface_normals()
	.compute_uv_coords();

	let mut surf_mesh = Mesh::new(PrimitiveTopology::TriangleList);
	surf_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, surf.vertices);
	surf_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, surf.normals);
	surf_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, surf.uv_coords);
	surf_mesh.set_indices(Some(Indices::U32(Vec::from(surf.indices.as_slice().unwrap()))));
	surf_mesh.generate_tangents().unwrap();

	_commands.insert_resource(Canvas3DInfo::default());

	_commands.spawn((PbrBundle {
		mesh: meshes.add(surf_mesh),
		material: materials.add(StandardMaterial {
			base_color: Color::Rgba {
				red: 0.8946, blue: 0.2716, green: 0.2426, alpha: 0.80
			}, 
			metallic: 0.5,
			perceptual_roughness: 0.2,
			reflectance: 0.5,	
			cull_mode: None,
			double_sided: true,
			alpha_mode: AlphaMode::Blend,
			//flip_normal_map_y: true,
			..default()
		}),
		//transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..default()
		},
		Wireframe,
		SurfaceComponent,
	));
	_commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::try_from(shape::Icosphere {
			radius: 3.0, 
			subdivisions: 32,
		}).unwrap()),
		material: materials.add(StandardMaterial {
			base_color: Color::hex("#ffd891").unwrap(),
			metallic: 1.0,
			perceptual_roughness: 0.5,
			..default()
		}),
		transform: Transform::from_xyz(0.0, 15.0, 0.0),
		..default()
	});

	_commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-10.0, 60.0, -30.0),
		point_light: PointLight {
			intensity: 300000.0,
			range: 140.0,
			..default()
		},
		..default()
	}); 

	_commands.insert_resource(AmbientLight {
		color: Color::hex("#ffffff").unwrap(),
		brightness: 0.2,
	});

    _commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0.0, 0.0, 40.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
			/*projection: OrthographicProjection {
				scale: 0.01,
				..default()
			}.into(), */
			..default()
	    },
		CameraControls::default()
		/*EnvironmentMapLight {
		} */
	));
}

#[derive(Component)]
struct CameraControls {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for CameraControls {
    fn default() -> Self {
        CameraControls {
            focus: Vec3::ZERO,
            radius: 40.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn camera_system(
	canvas_info: Res<Canvas3DInfo>,
    mut motion_event: EventReader<MouseMotion>,
    mut scroll_event: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut CameraControls, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for event in motion_event.iter() {
            rotation_move += event.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for event in motion_event.iter() {
            pan += event.delta;
        }
    }
    for event in scroll_event.iter() {
        scroll += event.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut controls, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            controls.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let delta_x = {
                let delta = rotation_move.x / canvas_info.width * std::f32::consts::PI * 2.0;
                if controls.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / canvas_info.height * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) 
					/ Vec2::new(canvas_info.width, canvas_info.height);
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * controls.radius;
            controls.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            controls.radius -= scroll * controls.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            controls.radius = f32::max(controls.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = controls.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, controls.radius));
			// looking_at has the camera look_at(origin) always
        }
    }
    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    motion_event.clear();
}

fn transformations_system(
	mut _commands: Commands, 
	mut query: Query<(&mut Transform, Entity), With<SurfaceComponent>>,
	key: Res<Input<KeyCode>>,
	mut canvas_info: ResMut<Canvas3DInfo>
) {
	let a = key.just_pressed(KeyCode::A);
	let ctrl = key.pressed(KeyCode::LControl);
	if a && ctrl {
		// must redraw
		if canvas_info.wireframe == true {
			for (transform, entity) in query.iter_mut() {
				_commands.entity(entity).remove::<Wireframe>();
			}	
			canvas_info.wireframe = false;
		} else {
			for (transform, entity) in query.iter_mut() {
				_commands.entity(entity).insert(Wireframe);
			}	
			canvas_info.wireframe = true;
		}
		println!("JIODJSOFOISDJFOJOSEJRO");	
	}
}

pub struct Canvas3D;

impl Plugin for Canvas3D {
	fn build(&self, app: &mut App) {
		println!("BUILD THIS");
		app.add_startup_system(canvas3d_startup)
		.add_plugin(WireframePlugin)
		.add_system(camera_system)
		.add_system(transformations_system);
	}
}

#[cfg(test)]
mod tests {
	use crate::fiber::canvas3d;
	#[test]
	fn test_ndarray_linspace() {
		let a = canvas3d::Surface::default();
		a.compute_points((-10.0, 10.0), (-10.0, 10.0), 20);

	}

	#[test]
	fn test_indices() {
		let a = canvas3d::Surface::default();
		a.compute_points((-10.0, 10.0), (-10.0, 10.0), 10)
			.compute_indices();
	}

	#[test]
	fn test_normals() {
		let a = canvas3d::Surface::default()
		.compute_points((-10.0, 10.0), (-10.0, 10.0), 10)
		.compute_indices()
		.compute_surface_normals();
	}
}