#[allow(unused_imports)]

#[macro_use] pub extern crate glium;

pub extern crate winit;
pub extern crate glutin;


pub extern crate cgmath;
pub type Matrix2 = cgmath::Matrix2<f64>;
pub type Matrix3 = cgmath::Matrix3<f64>;
pub type Matrix4 = cgmath::Matrix4<f64>;

mod renderer;
pub use renderer::Renderer;

pub mod widgets;
pub mod color;

#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Point,
}

impl Rect {
    pub fn coords(&self) -> (f32, f32, f32, f32) {
        (
            self.origin.x,                  // x1
            self.origin.y,                  // y1
            self.origin.x + self.size.w(),  // x2
            self.origin.y + self.size.h(),  // y2
        )
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Self{ x:x, y:y }}
    pub fn xy(&self) -> (f32, f32) { (self.x, self.y) }
    pub fn wh(&self) -> (f32, f32) { (self.x, self.y) }
    pub fn w(&self) -> f32 { self.x }
    pub fn h(&self) -> f32 { self.y }
}
