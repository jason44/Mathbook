use std::{borrow::BorrowMut, f32::consts::PI, ops::IndexMut};
use bevy::{
	prelude::*, 
	input::{
		ButtonState, keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel, MouseMotion}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings,
	render::mesh::{VertexAttributeValues, PrimitiveTopology, Indices},
	math::Vec3A,
};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use ndarray::{prelude::*, Zip};
use crate::fiber::ui::UiState;
use std::iter::FromIterator;

#[derive(Resource)]
struct Canvas3DInfo {
	// separate solids and surfaces because they 
	// are drawn with different frag shaders
	surfaces: Vec<Handle<Mesh>>,
	solids: Vec<Handle<Mesh>>,
}

pub struct Surface {
	xs: Array<f32, Ix1>,
	ys: Array<f32, Ix1>,
	zs: Array<f32, Ix2>,
	pub vertices: Vec<Vec3A>,
	pub indices: Array<u32, Ix1>,
	pub normals: Vec<Vec3A>,  
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
				//self.vertices.push(Vec3A::new(x_samples[i], y_samples[j], z_samples[(i, j)]));
				self.vertices.push(Vec3A::new(x_samples[i], y_samples[j], z_samples[(i, j)]));
			}
		}

		self.xs = x_samples;
		self.ys = y_samples;
		self.zs = z_samples;
		self.res = res as u32;
		println!("VERTICES\n{:?}\n-----------------------------------------", self.vertices);
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

		println!("INDICES\n{:?}\n---------------------------------", indices);
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

		println!("LEFT\n{:?}\n--------------------", left);
		println!("RIGHT\n{:?}\n--------------------", right);
		println!("UP\n{:?}\n--------------------", up);
		println!("DOWN\n{:?}\n--------------------", down);
		let vertices = &self.vertices;
		let mut crosses = Vec::<Vec3A>::with_capacity(nunv);
		for i in 0..nunv {
			/*println!("right: {}, left: {}", vertices[right[i] as usize], vertices[left[i] as usize]);
			println!("up: {}, down: {}", vertices[up[i] as usize], vertices[down[i] as usize]);
			println!("cross: {}\n-----------------\n", 
			(vertices[right[i] as usize] - vertices[left[i] as usize])
			.cross(vertices[up[i] as usize] - vertices[down[i] as usize])); */
			crosses.push((vertices[right[i] as usize] - vertices[left[i] as usize])
			.cross(vertices[up[i] as usize] - vertices[down[i] as usize]));
		}
		//println!("CROSSES\n{:?}\n-------------------------", crosses);
		//self.normals = crosses;
		for normal in crosses {
			self.normals.push(normal.normalize());
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
	.compute_points((-10.0, 10.0), (-10.0, 10.0), 100)
	.compute_indices()
	.compute_surface_normals();

	let mut surf_mesh = Mesh::new(PrimitiveTopology::TriangleList);
	surf_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, surf.vertices);
	surf_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, surf.normals);
	surf_mesh.set_indices(Some(Indices::U32(Vec::from(surf.indices.as_slice().unwrap()))));
	//surf_mesh.generate_tangents().expect("failed to generate surface tangents");	

	_commands.spawn(PbrBundle {
		mesh: meshes.add(surf_mesh),
		material: materials.add(StandardMaterial {
			base_color: Color::hex("#ffd891").unwrap(),
			metallic: 0.8,
			perceptual_roughness: 0.8,
			..default()
		}),
		//transform: Transform::from_xyz(0.0, 0.0, 0.0),
		//transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 1.), 1.5707)),
		..default()
	});
	_commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::try_from(shape::Icosphere {
			radius: 2.0, 
			subdivisions: 32,
		}).unwrap()),
		material: materials.add(StandardMaterial {
			base_color: Color::hex("#ffd891").unwrap(),
			metallic: 4.0,
			perceptual_roughness: 1.0,
			..default()
		}),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..default()
	});
	
	_commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-10.0, 60.0, -30.0),
		point_light: PointLight {
			intensity: 400000.0,
			range: 140.0,
			..default()
		},
		..default()
	});

    _commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0.0, 10.0, 40.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
			/*projection: OrthographicProjection {
				scale: 0.01,
				..default()
			}.into(), */
			..default()
	    },
		/*EnvironmentMapLight {
		} */
		CameraController::default()
	));
}

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 2.0,
            run_speed: 6.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}


pub struct Canvas3D;

impl Plugin for Canvas3D {
	fn build(&self, app: &mut App) {
		println!("BUILD THIS");
		app.add_startup_system(canvas3d_startup)
		.add_system(camera_controller);
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