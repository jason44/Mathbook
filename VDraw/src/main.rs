
//mod VDrawContext;

/* 
fn main() {
    println!("Hello, world!");
}
*/

extern crate cairo;
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;
fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)
        .expect("Couldn’t create surface");
    let context = Context::new(&surface)
        .expect("Couldn't create context");
    context.set_source_rgb(1.0, 0.0, 0.0);
    context.paint();
    let mut file = File::create("output.png")
        .expect("Couldn’t create file"); 
    surface.write_to_png(&mut file)
        .expect("Couldn’t write to png");
}