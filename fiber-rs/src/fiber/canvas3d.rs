use std::{borrow::BorrowMut, f32::consts::PI, ops::IndexMut};
use bevy::{
	prelude::*, 
	input::{
		ButtonState, keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings,
	render::mesh::VertexAttributeValues,
};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use ndarray::{prelude::*, Zip};
use crate::fiber::ui::UiState;
use std::iter::FromIterator;

pub struct Canvas3D;

impl Plugin for Canvas3D {
	fn build(&self, app: &mut App) {
		println!("BUILD THIS");
	}
}

pub struct Surface {
	xs: Array<f32, Ix1>,
	ys: Array<f32, Ix1>,
	zs: Array<f32, Ix2>,
	vertices : Vec<VertexAttributeValues>,
	indices : Array<f32, Ix1>,
	res: usize,
	func: fn(&f32, &&f32) -> f32,
}

impl Default for Surface {
	fn default() -> Self {
		Surface {
			xs: Array::<f32, _>::default(1),
			ys: Array::<f32, _>::default(1),
			zs: Array::<f32, _>::default((0, 0)),
			vertices: Vec::<VertexAttributeValues>::default(),
			indices: Array::<f32, _>::default(1),
			res: 40,
			func: |x, y| {x + *y},
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
			z_samples.push_row(Zip::from(&y_samples).and_broadcast(&arr0(_x)).map_collect(self.func).view()).unwrap();
		}

		self.vertices = Vec::with_capacity(z_samples.len());
		for i in 0..res {
			for j in 0..res {
				self.vertices.push(VertexAttributeValues::Float32(vec![x_samples[i], y_samples[j], z_samples[(i, j)]]));
			}
		}

		self.xs = x_samples;
		self.ys = y_samples;
		self.zs = z_samples;
		self.res = res;

		println!("VERTICES\n{:?}\n-----------------------------------------", self.vertices);
		/* 
		for i in 0..res {
			let z_row = zs.row(i);
			for j in 0..res {
				println!("[{}, {}, {}]", &x_samples[i], &&y_samples[j], z_row[j]);
			}
		}
		*/
		self
	}

	fn compute_indices(mut self) -> Self {
		let nu = self.res;
		let nv = self.res;
		let _grid = 
			Array::range(0.0, (nu * nv) as f32, 1.0);
		let grid = _grid.to_shape((nu, nv)).unwrap();
		let mut indices = Array1::<f32>::zeros(6 * (nu - 1) * (nv - 1));

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

		println!("{:?}\n---------------------------------", grid);
		println!("TOPLEFT\n{:?}\n---------------------------------", top_left);
		println!("BOTTOMLEFT\n{:?}\n---------------------------------", bottom_left);
		println!("INDICES\n{:?}\n---------------------------------", indices);
		self.indices = indices;	
		self
	}

}

fn canvas3d_startup(mut _commands: Commands) {
	//_commands.spawn
    _commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

}