pub mod vector3d;
pub mod color;
pub mod geometry;
pub mod camera;
pub mod loaders;

use std::f32;

use vector3d::{
    Vector3D, vec_get_length,
    vec_dot, vec_normalize,
    vec_sum, vec_sub, vec_multiplication, vec_hadamard
};

use geometry::{
    Line, Sphere, Plane, Intersect, Triangle
};

use color::{Material};

#[derive(Clone, Copy)]
pub struct PointLight {
    pub position: Vector3D,
    pub color: Vector3D,
    pub range: f32
}

impl PointLight {
    pub fn new (pos: Vector3D, color: Vector3D, range: f32) -> PointLight {
        return PointLight {
            position: pos,
            color: color,
            range: range
        };
    }
}


/**
 * World struct
 */
#[derive(Clone)]
pub struct World {
    pub materials: Vec<Material>,

    pub planes:  Vec<Plane>,
    pub shperes: Vec<Sphere>,

    pub objects: Vec<Triangle>,

    pub lights: Vec<PointLight>
}

impl World {
    pub fn new() -> World{
        return World {
            materials: Vec::new(),
            planes: Vec::new(),
            shperes: Vec::new(),

            objects: Vec::new(),

            lights: Vec::new()
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

        for obj in world.objects.iter() {
            if let Some(t) = obj.intersects(&current_line) {
                if t < hit_distance {
                    hit_distance = t;
                    final_material = obj.get_material_index();

                    next_origin = current_line.get_point(t);
                    next_normal = obj.get_normal(&next_origin);
                }
            }
        }

        if final_material != 0 {
            let material = world.materials.get(final_material).unwrap();

            // Reflection
            let reflection_coeficient: f32 = 2.0 * vec_dot(&line.direction, &next_normal);
            let reflection_correction = vec_multiplication(&next_normal, reflection_coeficient);
            let pure_reflection = vec_sub(&line.direction, &reflection_correction);

            let random_reflection = Vector3D::new_random(material.roughness);

            let reflection = vec_normalize(
                &vec_sum(
                    &pure_reflection,
                    &random_reflection
                )
            );

            // --------------------------------------
            // Light stuff
            // --------------------------------------

            let mut light_contribs: Vec<(f32, Vector3D)> = Vec::new();

            for light in world.lights.iter() {
                let point_to_light = vec_normalize(&vec_sub(&light.position, &next_origin));

                let mut coeficient: f32= 1.0;
                coeficient += vec_dot(
                    &next_normal,
                    &point_to_light
                );

                if coeficient < 0.1 {
                    coeficient = 0.1;
                }

                let surface_to_light : Line = Line::new(next_origin, point_to_light);
                let point_to_light_distance = vec_get_length(&vec_sub(&light.position, &next_origin));
                // Shadow calculation coeficient
                for obj in world.objects.iter() {
                    if let Some(t) = obj.intersects(&surface_to_light) {
                        if t < point_to_light_distance {
                            coeficient *= 0.25;
                            break;
                        }
                    }
                }

                let mut percent: f32 = point_to_light_distance / light.range;

                if percent > 1.0 { percent = 1.0; }

                let light_power = 1.0 - percent;

                light_contribs.push( (coeficient, vec_multiplication(&light.color, light_power)) );
            }

            // --------------------------------------
            // Color modification
            // --------------------------------------

            // Emission contribution
            result_color = vec_sum(&result_color, &vec_hadamard(&attenuation, &material.emision_color));

            // Light absortion
            attenuation = vec_hadamard(
                &attenuation,
                &vec_multiplication(
                    // More metalic, less attenuation by base color.
                    &material.base_color, 1.0 - (0.5 + (material.metalic * 0.5))
                )
            );

            // Lights contribution
            let mut coeficient_sum: f32 = 1.0;
            for contrib in light_contribs {
                let diffuse_light = vec_hadamard(&contrib.1, &attenuation);
                result_color = vec_sum(
                    &result_color,
                    &vec_multiplication(&diffuse_light, contrib.0)
                );

                coeficient_sum *= contrib.0;
            }
            attenuation = vec_multiplication(&attenuation, coeficient_sum);

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