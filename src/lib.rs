pub mod vector3d;
pub mod color;
pub mod geometry;

use std::f32;

use vector3d::{
    Vector3D, vec_get_length,
    vec_dot, vec_normalize,
    vec_sum, vec_sub, vec_multiplication, vec_hadamard
};

use geometry::{ Line, Sphere, Plane };

use color::{Material};

const MARGIN: f32 = 0.001f32;

/**
 * World struct
 */
pub struct World {
    pub materials: Vec<Material>,

    pub planes:  Vec<Plane>,
    pub shperes: Vec<Sphere>
}

impl World {
    pub fn new() -> World{
        return World {
            materials: Vec::new(),
            planes: Vec::new(),
            shperes: Vec::new()
        }
    }
}

/**
 * Trace party
 */
pub fn trace(world: &World, line: &Line, max_bounces: u32) -> (Vector3D, u32) {
    let mut hit_distance: f32 = f32::MAX;
    let mut final_material: usize = 0;

    // Color calculation
    let mut result_color: Vector3D = Vector3D::new_as_zero();
    let mut attenuation: Vector3D = Vector3D::new_as_one();

    // Bounces
    let mut current_line: Line = Line::from(line);
    let mut next_origin: Vector3D = Vector3D::new_as_zero();
    let mut next_normal: Vector3D = Vector3D::new_as_zero();

    let mut bounces_performed: u32 = 0;
    let mut i: u32 = 0;
    while i < max_bounces {
        i += 1;

        // Planes raycast
        for plane in world.planes.iter() {
            if let Some(t) = intersect_line_plane(&current_line, plane) {
                if t < hit_distance {
                    hit_distance = t;
                    final_material = plane.material_index;

                    next_origin = current_line.get_point(t);
                    next_normal = plane.n;
                }
            }
        }

        // Spheres raycast
        for sphere in world.shperes.iter() {
            if let Some(t) = intersect_line_sphere(&current_line, sphere) {
                if t < hit_distance {
                    hit_distance = t;
                    final_material = sphere.material_index;

                    next_origin = current_line.get_point(t);
                    next_normal = vec_normalize(&vec_sub(&next_origin, &sphere.o));
                }
            }
        }

        if final_material != 0 {
            let material = world.materials.get(final_material).unwrap();

            // Reflection
            let reflection_coeficient: f32 = 2.0 * vec_dot(&line.d, &next_normal);
            let reflection_correction = vec_multiplication(&next_normal, reflection_coeficient);
            let pure_reflection = vec_sub(&line.d, &reflection_correction);

            let random_reflection = Vector3D::new_random(material.roughness);

            let reflection = vec_normalize(
                &vec_sum(
                    &pure_reflection,
                    &random_reflection
                )
            );

            // --------------------------------------
            // Color modification
            // --------------------------------------

            // Cosine term for color reflection.
            let mut cos_term = 1.0;
            /* vec_dot(
                &vec_multiplication(&current_line.d, -1.0), &next_normal
            );*/

            if cos_term < 0.0 {
                cos_term = 0.0;
            }

            result_color = vec_sum(&result_color, &vec_hadamard(&attenuation, &material.emision_color));
            attenuation = vec_hadamard(
                &vec_multiplication(&attenuation, cos_term), &material.base_color
            );

            final_material = 0;

            // Create the next line
            current_line = Line::new(next_origin, reflection);
            bounces_performed += 1;
        } else {
            // Avoid keep raycasting if it doesn't hit anything.
            i = max_bounces;

            // FIXME: Assume we hit the sky
            let sky_material = world.materials.get(0).unwrap();
            result_color = vec_sum(&result_color, &vec_hadamard(&attenuation, &sky_material.emision_color));
        }
    }

    if vec_get_length(&result_color) > 1.73 {
        result_color = vec_multiplication(&vec_normalize(&result_color), 1.73);
    }

    return (result_color, bounces_performed); // world.materials.get(final_material).unwrap().base_color;
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

    if t < MARGIN {
        return None;
    }

    return Some(t);

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