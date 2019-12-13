use crate::vector3d::{
    Vector3D,
    vec_normalize, vec_dot, vec_cross,
    vec_sum, vec_sub, vec_multiplication
};

const MARGIN: f32 = 0.001f32;

pub trait Intersect {
    fn intersects(&self, line: &Line) -> Option<f32>;
    fn get_normal(&self, surface_point: &Vector3D) -> Vector3D;
    fn get_material_index(&self) -> usize;
}

// ================================================
// Line implementation
// ================================================

pub struct Line {
    pub origin: Vector3D,
    pub direction: Vector3D
}

impl Line {
    pub fn new(o: Vector3D, d: Vector3D) -> Line {
        return Line {origin: o, direction: d};
    }

    pub fn from(line: &Line) -> Line {
        return Line::new(line.origin, line.direction);
    }

    pub fn get_point(&self, t: f32) -> Vector3D {
        return vec_sum(&self.origin, &vec_multiplication(&self.direction, t));
    }
}

// ================================================
// Plane implementation
// ================================================
#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vector3D,
    pub point: Vector3D
}

impl Plane {
    pub fn new(n: Vector3D, p: Vector3D) -> Plane {
        return Plane {normal: vec_normalize(&n), point: p};
    }
}

impl Intersect for Plane {
    /**
     * Checks the intersection point on the given line and plane
     *
     * @param {Line} line
     *
     * @return {Option<f32>} The 't' line offset value or None.
     */
    fn intersects(&self, line: &Line) -> Option<f32> {
        let denom = vec_dot(&self.normal, &line.direction);

        if denom.abs() < MARGIN || denom >= 0.0 {
            return None;
        }

        let t = (- vec_dot(&self.normal, &line.origin) + vec_dot(&self.normal, &self.point)) / denom;

        if t < MARGIN {
            return None;
        }

        return Some(t);
    }

    fn get_normal(&self, _: &Vector3D) -> Vector3D {
        return self.normal;
    }

    fn get_material_index(&self) -> usize {
        return 0;
    }
}

// ================================================
// Sphere implementation
// ================================================
#[derive(Clone, Copy)]
pub struct Sphere {
    pub origin: Vector3D,
    pub radius: f32,

    pub material_index: usize
}

impl Sphere {
    pub fn new(o: Vector3D, r: f32, mat_index: usize) -> Sphere {
        return Sphere {origin: o, radius: r, material_index: mat_index};
    }
}

impl Intersect for Sphere {
    /**
     * Checks the intersection of the given line and the sphere.
     *
     * @param Line line
     *
     * @return {Option<f32>} The 't' line offset value or None.
     */
    fn intersects(&self, line: &Line) -> Option<f32> {
        // Quadratic ecuation
        // -b +- SQRT( b*b -4*a*c ) / 2*a

        let origin = vec_sub(&line.origin, &self.origin);

        let a: f32 = vec_dot(&line.direction, &line.direction);
        let b: f32 = 2.0 * vec_dot(&origin, &line.direction);
        let c: f32 = vec_dot(&origin, &origin) - self.radius * self.radius;

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

    fn get_normal(&self, surface_point: &Vector3D) -> Vector3D {
        let normal = vec_sub(&surface_point, &self.origin);

        return vec_normalize(&normal);
    }

    fn get_material_index(&self) -> usize {
        return self.material_index;
    }
}

// ================================================
// Triangle implementation
// ================================================

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vector3D,
    pub b: Vector3D,
    pub c: Vector3D,

    pub inner_plane: Plane,
    pub material_index: usize
}

impl Triangle {
    pub fn new(a: Vector3D, b: Vector3D, c: Vector3D, mat_index: usize) -> Triangle {
        let ab: Vector3D = vec_sub(&b, &a);
        let ac: Vector3D = vec_sub(&c, &a);

        let normal: Vector3D = vec_normalize(&vec_cross(&ac, &ab));

        let the_plane: Plane = Plane::new(normal, a);

        return Triangle {
            a: a, b: b, c: c,
            material_index: mat_index,
            inner_plane: the_plane
        }
    }
}

impl Intersect for Triangle {
    fn intersects(&self, line: &Line) -> Option<f32> {
        if let Some(t) = self.inner_plane.intersects(line) {
            let point: Vector3D = line.get_point(t);
            let tri_normal: Vector3D = self.get_normal(&point);

            // Calculates the cross product of the edge and the normal,
            //   if it's negative, the point is outside.
            let ab: Vector3D = vec_sub(&self.b, &self.a);
            let pa: Vector3D = vec_sub(&point, &self.a);

            let diff = vec_dot(&tri_normal, &vec_cross(&pa, &ab));
            if diff <= 0.0 {
                return None;
            }

            let bc: Vector3D = vec_sub(&self.c, &self.b);
            let pb: Vector3D = vec_sub(&point, &self.b);

            let diff = vec_dot(&tri_normal, &vec_cross(&pb, &bc));
            if diff <= 0.0 {
                return None;
            }

            let ca: Vector3D = vec_sub(&self.a, &self.c);
            let pc: Vector3D = vec_sub(&point, &self.c);

            let diff = vec_dot(&tri_normal, &vec_cross(&pc, &ca));
            if diff <= 0.0 {
                return None;
            }

            return Some(t);
        }
        return None;
    }

    fn get_normal(&self, _: &Vector3D) -> Vector3D {
        return self.inner_plane.normal;
    }

    fn get_material_index(&self) -> usize {
        return self.material_index;
    }
}

// ================================================
// Triangle implementation
// ================================================

pub struct Mesh {
    pub triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            triangles: Vec::new()
        };
    }
}