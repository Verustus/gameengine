use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use super::math;

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl From<Vec3> for Vec<f32> { fn from(src: Vec3) -> Vec<f32> { vec![src.x, src.y, src.z] } }
impl From<Vec<f32>> for Vec3 { fn from(src: Vec<f32>) -> Vec3 { Vec3 { x: src[0], y: src[1], z: src[2]} } }
impl From<Vec3> for [f32; 3] { fn from(src: Vec3) -> [f32; 3] { [src.x, src.y, src.z] } }
impl From<[f32; 3]> for Vec3 { fn from(src: [f32; 3]) -> Vec3 { Vec3 { x: src[0], y: src[1], z: src[2]} } }
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output { Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z } }
}
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output { Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output { Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z } }
}
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output { Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z } }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

#[derive(Default)]

pub struct Shape {
    verticles: Vec<Vertex>,
    indices: Vec<u32>,
    texture: Option<glium::texture::Texture2d>,
    vertex_shader: Option<String>,
    fragment_shader: Option<String>
}

#[derive(Default, Copy, Clone)]
pub struct Vertex {
    default_position: Vec3,
    position: Option<Vec3>,
    texture_coordinates: [f32; 2]
}

trait VertexMath {
    fn rotate_around(&mut self, rotation: Vec3, rotation_point: Vec3);
    fn move_to(&mut self, position: Vec3);
    fn get_position(&self) -> Vec3;
    fn reset(&mut self);
}

impl VertexMath for Vertex {
    fn rotate_around(&mut self, rotation: Vec3, rotation_point: Vec3) {
        self.position = Some(math::rotate(*self.position.get_or_insert(self.default_position)
                        - rotation_point, rotation) + rotation_point);
    }

    fn move_to(&mut self, position: Vec3) {
        self.position = Some(position.into());
    }

    fn get_position(&self) -> Vec3 {
        if self.position.is_some() { self.position.unwrap().into() } else { self.default_position.into() }
    }

    fn reset(&mut self) {
        self.position = None;
    }
}

#[derive(Default, Copy, Clone)]
pub struct RenderVertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2]
}

impl From<Vertex> for RenderVertex {
    fn from(mut src: Vertex) -> RenderVertex {
        RenderVertex {
            position: (*src.position.get_or_insert(src.default_position)).into(),
            texture_coords: src.texture_coordinates
        }
    }
}

pub trait Rotate {
    fn rotate(&mut self, rotation: Vec3, rotation_point: Vec3);
}

impl Rotate for Vec<RenderVertex> {
    fn rotate(&mut self, rotation: Vec3, rotation_point: Vec3) {
        for vertex in self {
            vertex.position = <[f32; 3]>::from(math::rotate(Vec3::from(vertex.position) - rotation_point, rotation) + rotation_point);
        }
    }
}