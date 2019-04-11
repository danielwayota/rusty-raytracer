use crate::vector3d::{
    Vector3D,
    vec_sum, vec_multiplication
};

/**
 * Line implementation
 */
pub struct Line {
    pub o: Vector3D,
    pub d: Vector3D
}

impl Line {
    pub fn new(o: Vector3D, d: Vector3D) -> Line {
        return Line {o: o, d: d};
    }

    pub fn from(line: &Line) -> Line {
        return Line::new(line.o, line.d);
    }

    pub fn get_point(&self, t: f32) -> Vector3D {
        return vec_sum(&self.o, &vec_multiplication(&self.d, t));
    }
}

/**
 * Plane implementation
 */
pub struct Plane {
    pub n: Vector3D,
    pub d: f32,

    pub material_index: usize
}

impl Plane {
    pub fn new(n: Vector3D, d: f32, mat_index: usize) -> Plane {
        return Plane {n: n, d: d, material_index: mat_index};
    }
}

/**
 * Sphere implementation
 */

pub struct Sphere {
    pub o: Vector3D,
    pub r: f32,

    pub material_index: usize
}

impl Sphere {
    pub fn new(o: Vector3D, r: f32, mat_index: usize) -> Sphere {
        return Sphere {o: o, r: r, material_index: mat_index};
    }
}