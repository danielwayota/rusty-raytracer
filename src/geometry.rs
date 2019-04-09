use crate::vector3d::Vector3D;

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
}

/**
 * Plane implementation
 */
pub struct Plane {
    pub n: Vector3D,
    pub d: f32
}

impl Plane {
    pub fn new(n: Vector3D, d: f32) -> Plane {
        return Plane {n: n, d: d};
    }
}

/**
 * Sphere implementation
 */

pub struct Sphere {
    pub o: Vector3D,
    pub r: f32
}

impl Sphere {
    pub fn new(o: Vector3D, r: f32) -> Sphere {
        return Sphere {o: o, r: r};
    }
}