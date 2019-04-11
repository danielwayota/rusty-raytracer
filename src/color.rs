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

fn clamp<T>(num: T, min: T, max: T) -> T
where
    T: PartialOrd
{
    if num > max { return max; }
    if num < min { return min; }
    return num;
}

pub fn float_color_from_bytes(r: u8, g: u8, b: u8) -> Vector3D {
    return Vector3D::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0
    );
}

pub fn color_to_u32(c: &Vector3D) -> u32 {
    let r: u32 = (c.x * 255.0) as u32;
    let g: u32 = (c.y * 255.0) as u32;
    let b: u32 = (c.z * 255.0) as u32;

    return (r << 16) | (g << 8) | (b);
}

pub fn linear_color_to_sRGB(color: &Vector3D) -> Vector3D {
    return Vector3D::new(
        gamma_correct(color.x),
        gamma_correct(color.y),
        gamma_correct(color.z)
    );
}

fn gamma_correct(input: f32) -> f32 {
    let l = clamp(input, 0.0, 1.0);

    if l >= 0.0 && l <= 0.0031308 {
        return l * 12.92;
    } else if l > 0.0031308 && l < 1.0 {
        return 1.055 * l.powf(1.0 / 2.4) - 0.055;
    }

    return l;
}