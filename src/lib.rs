pub mod vector3d;
pub mod color;
pub mod geometry;

use std::f32;

use vector3d::{
    Vector3D,
    vec_dot,
    vec_sum, vec_sub, vec_multiplication
};

use geometry::{
    Line, Sphere, Plane
};

const MARGIN: f32 = 0.001f32;

/**
 * World struct
 */
pub struct World {
    pub sky_color: Vector3D,

    pub planes:  Vec<Plane>,
    pub shperes: Vec<Sphere>
}

impl World {
    pub fn new() -> World{
        return World {
            sky_color: Vector3D::new(0.718, 0.765, 0.953),
            planes: Vec::new(),
            shperes: Vec::new()
        }
    }
}

/**
 * Trace party
 */
pub fn trace(world: &World, line: &Line) -> Vector3D {
    let mut hit_distance: f32 = f32::MAX;

    let mut final_color = world.sky_color;

    for plane in world.planes.iter() {
        if let Some(t) = intersect_line_plane(&line, plane) {
            if t < hit_distance {
                hit_distance = t;
                final_color = Vector3D::new(0.5, 0.2, 0.2);
            }
        }
    }

    for sphere in world.shperes.iter() {
        if let Some(t) = intersect_line_sphere(&line, sphere) {
            if t < hit_distance {
                hit_distance = t;
                final_color = Vector3D::new(0.2, 0.2, 0.5);
            }
        }
    }


    return final_color;
}

/**
 * Intersection functions
 */

pub fn intersect_line_plane(line: &Line, plane: &Plane) ->Option<f32> {
    let denom = vec_dot(&plane.n, &line.d);

    if  denom.abs() < MARGIN {
        return None;
    }

    let t = (- vec_dot(&plane.n, &line.o) - plane.d) / denom;

    if t < 0.0 {
        return None;
    }

    return Some(t);

}

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

    return Some(t);
}