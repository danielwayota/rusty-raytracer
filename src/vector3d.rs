/**
 * Vector Class implementation
 */
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
}

impl ToString for Vector3D {
    fn to_string(&self) -> String {
        return format!("({}, {}, {})", self.x, self.y, self.z);
    }
}

/**
 * Usefull Constants
 */

pub const I: Vector3D = Vector3D { x: 1.0, y: 0.0, z: 0.0 };
pub const J: Vector3D = Vector3D { x: 0.0, y: 1.0, z: 0.0 };
pub const K: Vector3D = Vector3D { x: 0.0, y: 0.0, z: 1.0 };

/**
 * Vector transformation functions
 */
pub fn get_length(v: &Vector3D) -> f32 {
    let sum: f32 = v.x * v.x + v.y * v.y + v.z * v.z;
    return sum.sqrt();
}

pub fn vec_normalize(v: &Vector3D) -> Vector3D {
    let length: f32 = get_length(&v);

    return vec_division(&v, length);
}

pub fn vec_sum(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.x + v.x,
        u.y + v.y,
        u.z + v.z
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

pub fn vec_linear_mult(u: &Vector3D, v: &Vector3D) -> Vector3D {
    return Vector3D::new(
        u.x * v.x,
        u.y * v.y,
        u.z * v.z
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length() {
        let v: Vector3D = Vector3D::new(1f32, 1f32, 1f32);

        let l: f32 = get_length(&v);
        assert!(l < 2f32 && l > 1f32);
    }

    #[test]
    fn vector_other_length() {
        let v: Vector3D = Vector3D::new(42f32, 42f32, 42f32);

        let l = get_length(&v);
        assert!(l > 72f32 && l < 73f32);
    }

    #[test]
    fn vector_vec_normalize() {
        let v: Vector3D = Vector3D::new(42f32, 42f32, 42f32);

        let n: Vector3D = vec_normalize(&v);

        let l = get_length(&n);
        assert!(l > 0.9f32 && l < 1.1f32);
    }

    #[test]
    fn vec_dot_product() {
        let u: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
        let v: Vector3D = Vector3D::new(-1.0, 0.0, 0.0);

        assert_eq!(vec_dot(&u, &v), -1.0);

        let u: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
        let v: Vector3D = Vector3D::new(1.0, 0.0, 0.0);

        assert_eq!(vec_dot(&u, &v), 1.0);

        let u: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
        let v: Vector3D = Vector3D::new(0.0, 1.0, 0.0);

        assert_eq!(vec_dot(&u, &v), 0.0);
    }
}