use crate::vector3d::Vector3D;

pub struct Material {
    pub base_color: Vector3D,
    pub emision_color: Vector3D,
    pub roughness: f32
}

impl Material {
    pub fn new(base: Vector3D, emit: Vector3D, roughness: f32) -> Material {
        return Material {
            base_color: base,
            emision_color: emit,
            roughness: roughness
        };
    }

    pub fn new_base(base: Vector3D) -> Material {
        return Material::new(base, Vector3D::new_as_zero(), 0.25);
    }

    pub fn new_light(emit: Vector3D) -> Material {
        return Material::new(emit, emit, 0.8);
    }
}

/**
 * Converts three bytes in a color in linear color space.
 * 
 * @param {u8} r
 * @param {u8} g
 * @param {u8} b
 * 
 * @return {Vector3D}
 */
pub fn float_color_from_bytes(r: u8, g: u8, b: u8) -> Vector3D {
    return Vector3D::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0
    );
}
/**
 * Converts a floating point RBG color to 32bit integer.
 * 
 * @param {Vector3D} c The floating point color
 * @return {u32} 32bit color
 */
pub fn color_to_u32(c: &Vector3D) -> u32 {
    let r: u32 = (c.x * 255.0) as u32;
    let g: u32 = (c.y * 255.0) as u32;
    let b: u32 = (c.z * 255.0) as u32;

    return (r << 16) | (g << 8) | (b);
}

/**
 * Converts a linear color to the sRBG color space.
 * 
 * @param {Vector3D} color
 * @return {Vector3D} Color in sRBG color space.
 */
pub fn linear_color_to_srgb(color: &Vector3D) -> Vector3D {
    return Vector3D::new(
        gamma_correct(color.x),
        gamma_correct(color.y),
        gamma_correct(color.z)
    );
}

/**
 * Uses the linear to sRBG formula to a single value
 * 
 * @param {f32} input
 * 
 * @return {f32}
 */
fn gamma_correct(input: f32) -> f32 {
    // Clamp the input
    let mut l = input;
    if input > 1.0 { l = 1.0; }
    if input < 0.0 { l = 0.0; }

    if l >= 0.0 && l <= 0.0031308 {
        return l * 12.92;
    } else if l > 0.0031308 && l < 1.0 {
        return 1.055 * l.powf(1.0 / 2.4) - 0.055;
    }

    return l;
}