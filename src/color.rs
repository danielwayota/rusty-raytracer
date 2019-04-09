use crate::vector3d::Vector3D;

fn clamp<T>(num: T, min: T, max: T) -> T
where
    T: PartialOrd
{
    if num > max { return max; }
    if num < min { return min; }
    return num;
}


pub fn color_to_u32(c: &Vector3D) -> u32 {
    let r: u32 = clamp(c.x * 255.0, 0.0, 255.0) as u32;
    let g: u32 = clamp(c.y * 255.0, 0.0, 255.0) as u32;
    let b: u32 = clamp(c.z * 255.0, 0.0, 255.0) as u32;

    return (r << 16) | (g << 8) | (b);
}