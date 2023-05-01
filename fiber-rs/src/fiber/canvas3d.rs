use std::{borrow::BorrowMut, f32::consts::PI};
use bevy::{
	prelude::*, 
	input::{
		ButtonState, keyboard::KeyboardInput,
		mouse::{MouseButtonInput, MouseWheel}, 
	},
	window::PrimaryWindow, window::WindowResized,
	winit::WinitSettings
};
use bevy_prototype_lyon::{prelude::*, shapes::*};
use ndarray::{prelude::*, Zip};
use crate::fiber::ui::UiState;

pub struct Canvas3D;


impl Plugin for Canvas3D {
	fn build(&self, app: &mut App) {
		println!("BUILD THIS");
	}
}

#[derive(Default)]
struct Vertex {
	x: f32, y:f32, z:f32
}

#[derive(Default)]
struct Index{
	x: f32, y:f32, z:f32
}


pub struct Surface {
	xs: Array<f32, Ix1>,
	ys: Array<f32, Ix1>,
	zs: Array<f32, Ix2>,
	vertices : Array<Vertex, Ix2>,
	indices : Array<Index, Ix2>,
	res: usize,
	func: fn(&f32, &&f32) -> f32,
}

impl Default for Surface {
	fn default() -> Self {
		Surface {
			xs: Array::<f32, _>::default(1),
			ys: Array::<f32, _>::default(1),
			zs: Array::<f32, _>::default((0, 0)),
			vertices: Array::<Vertex, _>::default((0, 0)),
			indices: Array::<Index, _>::default((0, 0)),
			res: 40,
			func: |x, y| {x + *y},
		}
	}
}
enum ComputeResult {
	Ok,
	Err,	
}

impl Surface {
	pub fn new() {

	}

	pub fn compute_points(mut self, x_range: (f32, f32), y_range: (f32, f32), res: usize) {
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
		self.xs = x_samples;
		self.ys = y_samples;
		self.zs = z_samples;
		/* 
		for i in 0..res {
			let z_row = zs.row(i);
			for j in 0..res {
				println!("[{}, {}, {}]", &x_samples[i], &&y_samples[j], z_row[j]);
			}
		}
		*/

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
}