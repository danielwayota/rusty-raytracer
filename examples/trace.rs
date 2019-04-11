extern crate raytracer;
extern crate minifb;

extern crate bmp;
use bmp::{Image, Pixel};

use minifb::{Window, WindowOptions, Scale};

use raytracer::{
    trace, World
};

use raytracer::geometry::{
    Line, Sphere, Plane
};

use raytracer::vector3d::{
    Vector3D,
    J,
    vec_sum, vec_sub, vec_multiplication, vec_division,
    vec_cross, vec_normalize
};
use raytracer::color::{Material, color_to_u32, linear_color_to_sRGB};


macro_rules! screen_to_percent {
    ($index: expr, $size: expr) => {
        2.0 * ($index as f32 / ($size-1) as f32) - 1.0
    };
}

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 640;
    const SCREEN_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

    // Window setup
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut float_buffer: Vec<Vector3D> = vec![Vector3D::new_as_zero(); WIDTH * HEIGHT];

    let options = WindowOptions::default();
    // options.scale = Scale::X32;
    let mut window = Window::new("Raycaster - The Begining", WIDTH, HEIGHT, options).unwrap();

    // Raycast stuff

    // Le camera
    let camera_pos: Vector3D = Vector3D::new(0.0, 0.5, 8.0);
    let camera_look_point: Vector3D = Vector3D::new(0.0, 0.5, 0.0);
    let camera_dir: Vector3D = vec_normalize(&vec_sub(&camera_look_point, &camera_pos));

    // Camera proyection plane
    let proj_right = vec_normalize(&vec_cross(&camera_dir, &J));
    let proj_up =  vec_normalize(&vec_cross(&proj_right, &camera_dir));

    let proj_plane_position = vec_sum(&camera_pos, &vec_multiplication(&camera_dir, 2.0));

    // World objects
    let mut world = World::new();

    world.materials.push(
        Material::new(
            Vector3D::new(0.25, 0.75, 0.25),
            Vector3D::new_as_zero(),
            0.8
        )
    );
    world.materials.push(
        Material::new(
            Vector3D::new(0.2, 0.257, 0.835),
            Vector3D::new_as_zero(),
            0.25
        )
    );
    world.materials.push(
        Material::new_light(
            Vector3D::new(1.0, 0.1, 0.1)
        )
    );

    world.materials.push(
        Material::new_light(Vector3D::new(0.1, 1.0, 0.1))
    );

    world.planes.push(
        Plane::new(Vector3D::new(0.0, 1.0, 0.0), 0.0, 1)
    );

    world.shperes = vec![
        Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0, 2),
        Sphere::new(Vector3D::new(2.0, 0.0, 2.0), 1.0, 3)
    ];

    // Display the window
    let mut j = 0;
    while window.is_open() {
        // Compose the image
        if j < HEIGHT {
            let film_y = screen_to_percent!(j, HEIGHT) * -1.0;

            // Horizontal scan line
            for i in 0..WIDTH {
                let film_x = screen_to_percent!(i, WIDTH) * SCREEN_RATIO;

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
                let mut color: Vector3D = Vector3D::new_as_zero();
                let samples: usize = 8;

                let single_color_contribution: f32 = 1.0 / samples as f32;

                for _ in 0..samples {
                    let trace_color = trace(&world, &line, 8);

                    color = vec_sum(&color, &vec_multiplication(&trace_color, single_color_contribution));
                }

                let index = j * WIDTH + i;

                buffer[index] = color_to_u32(&linear_color_to_sRGB(&color));
                float_buffer[index] = color;
            }

            j += 1;
            
        }

        window.update_with_buffer(&buffer).unwrap();
    }

    let mut final_image = Image::new(WIDTH as u32, HEIGHT as u32);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let color: Vector3D = float_buffer[j * WIDTH + i];

            final_image.set_pixel(
                i as u32, j as u32,
                Pixel::new(
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8
                )
            );
        }
    }

    let _ = final_image.save("result.bmp");
}

 