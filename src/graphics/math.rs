use ndarray::prelude::*;
use super::types::Vec3;

pub fn rotate(position: Vec3, rotation: Vec3) -> Vec3 {
    let vector_position = arr1(&[position.x, position.y, position.z, 0.0]);

    let x_cos: f32 = rotation.x.cos();
    let x_sin: f32 = rotation.x.sin();
    let x_mat = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, x_cos, -x_sin, 0.0],
        [0.0, x_sin, x_cos, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    
    let y_cos: f32 = rotation.y.cos();
    let y_sin: f32 = rotation.y.sin();
    let y_mat = arr2(&[
        [y_cos, 0.0, y_sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-y_sin, 0.0, y_cos, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);
    
    let z_cos: f32 = rotation.z.cos();
    let z_sin: f32 = rotation.z.sin();
    let z_mat = arr2(&[
        [z_cos, -z_sin, 0.0, 0.0],
        [z_sin, z_cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    return vector_position.dot(&x_mat.dot(&y_mat.dot(&z_mat))).to_vec().into();
}