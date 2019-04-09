extern crate raytracer;
extern crate minifb;

use minifb::{Window, WindowOptions, Scale};

use raytracer::{
    trace, World
};

use raytracer::geometry::{
    Line, Sphere, Plane
};

use raytracer::vector3d::{
    Vector3D,
    I, J, K,
    vec_sum, vec_sub, vec_multiplication, vec_division,
    vec_cross, vec_normalize
};
use raytracer::color::color_to_u32;

fn main() {
    const WIDTH: usize = 320;
    const HEIGHT: usize = 320;

    // Window setup
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut options = WindowOptions::default();
    options.scale = Scale::X2;
    let mut window = Window::new("Raycaster - The Begining", WIDTH, HEIGHT, options).unwrap();

    // Raycast stuff

    // Le camera
    let camera_pos: Vector3D = Vector3D::new(0.0, 0.5, 10.0);
    let camera_look_point: Vector3D = Vector3D::new(0.0, 0.5, 0.0);
    let camera_dir: Vector3D = vec_normalize(&vec_sub(&camera_look_point, &camera_pos));

    // Camera proyection plane
    let proj_right = vec_normalize(&vec_cross(&camera_dir, &J));
    let proj_up =  vec_normalize(&vec_cross(&proj_right, &camera_dir));

    let proj_plane_position = vec_sum(&camera_pos, &vec_multiplication(&camera_dir, 2.0));

    // World objects
    let mut world = World::new();

    world.planes.push(
        Plane::new(Vector3D::new(0.0, 1.0, 0.0), 0.0)
    );

    world.shperes = vec![
        Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0),
        Sphere::new(Vector3D::new(2.0, 0.0, 0.0), 0.7),
        Sphere::new(Vector3D::new(-2.0, 1.0, 1.0), 0.25)
    ];

    let mut running = true;
    while running {
        for j in 0..HEIGHT {    
            let film_y = (2.0 * (j as f32 / (HEIGHT-1) as f32) - 1.0) * -1.0;

            for i in 0..WIDTH {
                let film_x = 2.0 * (i as f32 / (WIDTH-1) as f32) - 1.0;

                // World space offset vectors
                let film_right = vec_multiplication(&proj_right, film_x);
                let film_up = vec_multiplication(&proj_up, film_y);
                
                // film plane combination
                let film_offset = vec_sum(&film_right, &film_up);
                let film_plane_point = vec_sum(&film_offset, &proj_plane_position);

                // Ray
                let origin = camera_pos;
                let direction = vec_normalize(&vec_sub(&film_plane_point, &origin));

                let line: Line = Line::new(origin, direction);

                // Raytrace stuff
                let index = j * WIDTH + i;
                buffer[index] = color_to_u32(&trace(&world, &line));
            }

        }

        window.update_with_buffer(&buffer).unwrap();
        running = window.is_open();
    }
}

 