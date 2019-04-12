use crate::vector3d::{
    Vector3D, J,
    vec_sub, vec_sum, vec_multiplication,
    vec_normalize, vec_cross
};

use crate::geometry::{Line};

pub struct Camera {
    pub position: Vector3D,
    pub look_direction: Vector3D,

    pub up: Vector3D,
    pub right: Vector3D,
    pub projection_plane_position: Vector3D
}

impl Camera {
    /**
     * Camera default contructor.
     * 
     * @param {Vector3D} pos The camera position in space.
     * @param 
     */
    pub fn new (pos: Vector3D, target: Vector3D, plane_t: f32) -> Camera {
        let camera_dir: Vector3D = vec_normalize(&vec_sub(&target, &pos));

        // Camera proyection plane
        let proj_right = vec_normalize(&vec_cross(&camera_dir, &J));
        let proj_up =  vec_normalize(&vec_cross(&proj_right, &camera_dir));

        let proj_plane_position = vec_sum(&pos, &vec_multiplication(&camera_dir, plane_t));

        return Camera {
            position: pos,
            look_direction: camera_dir,

            up: proj_up,
            right: proj_right,

            projection_plane_position: proj_plane_position

        };
    }

    pub fn screen_point_to_projection_plane(x: usize, width: usize, y: usize, height: usize) {

    }
}