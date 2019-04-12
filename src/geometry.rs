use crate::vector3d::{
    Vector3D,
    vec_normalize, vec_dot,
    vec_sum, vec_sub, vec_multiplication
};

const MARGIN: f32 = 0.001f32;

// ================================================
// Line implementation
// ================================================

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

// ================================================
// Plane implementation
// ================================================

pub struct Plane {
    pub n: Vector3D,
    pub d: f32,

    pub material_index: usize
}

impl Plane {
    pub fn new(n: Vector3D, d: f32, mat_index: usize) -> Plane {
        return Plane {n: vec_normalize(&n), d: d, material_index: mat_index};
    }
}

/**
 * Checks the intersection point on the given line and plane
 * 
 * @param {Line} line
 * @param {Plane} plane
 * 
 * @return {Option<f32>} The 't' line offset value or None.
 */
pub fn intersect_line_plane(line: &Line, plane: &Plane) ->Option<f32> {
    let denom = vec_dot(&plane.n, &line.d);

    if  denom.abs() < MARGIN {
        return None;
    }

    let t = (- vec_dot(&plane.n, &line.o) - plane.d) / denom;

    if t < MARGIN {
        return None;
    }

    return Some(t);
}

// ================================================
// Sphere implementation
// ================================================

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

/**
 * Checks the intersection of the given line and the sphere.
 * 
 * @param Line line
 * @param Sphere shpere
 */ 
pub fn intersect_line_sphere(line: &Line, shpere: &Sphere) -> Option<f32> {
    // Quadratic ecuation
    // -b +- SQRT( b*b -4*a*c ) / 2*a

    let origin = vec_sub(&line.o, &shpere.o);

    let a: f32 = vec_dot(&line.d, &line.d);
    let b: f32 = 2.0 * vec_dot(&origin, &line.d);
    let c: f32 = vec_dot(&origin, &origin) - shpere.r * shpere.r;

    if a.abs() < MARGIN {
        return None;
    }

    let root: f32 = b*b - 4.0 * a * c;

    // Sqrt becomes imaginary
    if root < 0.0 {
        return None;
    }

    let tn: f32 = (- b - root.sqrt()) / 2.0 * a;
    let tp: f32  = (- b + root.sqrt()) / 2.0 * a;

    let mut t: f32 = tp;
    if tn > 0.0 && tn < tp {
        t = tn;
    }

    if t < MARGIN {
        return None;
    }

    return Some(t);
}