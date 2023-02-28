/*
 * 
 */
extern crate cairo;

pub use std::fs::File;

use cairo::{ImageSurface, Format, Context};

mod VDrawContext {
	pub struct VDrawContext {
		surface: ImageSurface::PdfSurface,
		format: Format,
		ctx: Context,
	}
	impl VDrawContext {
		pub fn new() -> VDrawContext {
			self.surface = ImageSurface::PdfSurface::create(Format::ARgb32, 600, 600)
				.expect("failed to create surface");
			self.context = Context::new(&sufrace);
			self.context.set_source_rgb(1.0, 0.0, 0.0);
			self.context.paint();
			
			let mut file = File::create("output.pdf")
				.expect("could not create file");

		}
	}
}


