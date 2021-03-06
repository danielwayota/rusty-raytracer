extern crate rand;

use rand::prelude::*;

// ================================================
// 3D Vector implementation
// ================================================
#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D{
        return Vector3D { x: x, y: y, z: z };
    }

    pub fn new_as_zero() -> Vector3D {
        return Vector3D::new(0.0, 0.0, 0.0);
    }

    pub fn new_as_one() -> Vector3D {
        return Vector3D::new(1.0, 1.0, 1.0);
    }

    pub fn new_random(multiplier: f32) -> Vector3D {
        let mut rng = rand::thread_rng();
        let phi = rng.gen::<f32>() * std::f32::consts::PI;
        let cos_theta = rng.gen::<f32>() * 2.0 - 1.0;

        let theta = cos_theta.acos();

        let tmp = Vector3D::new(
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos()
        );

        return vec_multiplication(&tmp, multiplier);
    } 
}

impl ToString for Vector3D {
    fn to_string(&self) -> String {
        return format!("({}, {}, {})", self.x, self.y, self.z);
    }
}

// ================================================
// Cartesian unit vectors
// ================================================

pub const I: Vector3D = Vector3D { x: 1.0, y: 0.0, z: 0.0 };
pub const J: Vector3D = Vector3D { x: 0.0, y: 1.0, z: 0.0 };
pub const K: Vector3D = Vector3D { x: 0.0, y: 0.0, z: 1.0 };


// ================================================
// Vector transformations
// ================================================

pub fn vec_get_length(v: &Vector3D) -> f32 {
    let sum: f32 = v.x * v.x + v.y * v.y + v.z * v.z;
    return sum.sqrt();
}

pub fn vec_normalize(v: &Vector3D) -> Vector3D {
    let length: f32 = vec_get_length(&v);

    return vec_division(&v, length);
}

pub fn vec_sum(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.x + v.x,
        u.y + v.y,
        u.z + v.z
    );
}

pub fn vec_sum_components(u: &Vector3D, x: f32, y: f32, z: f32) -> Vector3D {
    return Vector3D::new(
        u.x + x,
        u.y + y,
        u.z + z
    );
}

pub fn vec_sub(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.x - v.x,
        u.y - v.y,
        u.z - v.z
    );
}

pub fn vec_division(u: &Vector3D, scalar: f32) -> Vector3D {
    return Vector3D::new(
        u.x / scalar,
        u.y / scalar,
        u.z / scalar
    );
}

pub fn vec_multiplication(u: &Vector3D, scalar: f32) -> Vector3D {
    return Vector3D::new(
        u.x * scalar,
        u.y * scalar,
        u.z * scalar
    );
}

pub fn vec_dot(u: &Vector3D, v: &Vector3D) -> f32 {
    let x: f32 = u.x * v.x;
    let y: f32 = u.y * v.y;
    let z: f32 = u.z * v.z;

    return x + y + z;
}

pub fn vec_cross(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x
    );
}

pub fn vec_hadamard(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.x * v.x,
        u.y * v.y,
        u.z * v.z
    );
}
