use std::io::Result as IOResult;
use std::time::Instant;

extern crate bmp;
use bmp::{Image, Pixel};

extern crate minifb;
use minifb::{
    Window, WindowOptions, Scale,
    Menu
};

extern crate rand;
use rand::prelude::*;

extern crate raytracer;
use raytracer::{
    trace, World
};

use raytracer::geometry::{
    Line, Sphere, Plane
};

use raytracer::vector3d::{
    Vector3D,
    J,
    vec_sum, vec_sum_components, vec_sub, vec_multiplication,
    vec_cross, vec_normalize
};
use raytracer::color::{
    Material, color_to_u32,
    float_color_from_bytes, linear_color_to_sRGB
};


macro_rules! screen_to_percent {
    ($index: expr, $size: expr) => {
        2.0 * ($index as f32 / ($size-1) as f32) - 1.0
    };
}


fn save_buffer_to_bmp(buffer: &Vec<Vector3D>, img_width: u32, img_height: u32, file_name: &str) -> IOResult<()> {
    let mut final_image = Image::new(img_width, img_height);

    for j in 0..img_height {
        for i in 0..img_width {
            let color: Vector3D = buffer[(j * img_width + i) as usize];

            final_image.set_pixel(
                i, j,
                Pixel::new(
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8
                )
            );
        }
    }

    return final_image.save(file_name);
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

    // Menus
    let mut save_menu = Menu::new("File").unwrap();
    save_menu.add_item("Save Image", 42).build();
    window.add_menu(&save_menu);

    // Random
    let mut rng = rand::thread_rng();

    // Floor
    world.materials.push(
        Material::new(
            float_color_from_bytes(88, 117, 167),
            Vector3D::new_as_zero(),
            0.8
        )
    );
    world.materials.push(
        Material::new(
            float_color_from_bytes(10, 153, 153),
            Vector3D::new_as_zero(),
            0.8
        )
    );
    world.materials.push(
        Material::new_light(
            float_color_from_bytes(216, 36, 23)
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

    // Stats
    let mut finised = false;
    let mut num_rays: u64 = 0;

    let time = Instant::now();

    // Display the window
    let mut j = 0;
    while window.is_open() {

        window.is_menu_pressed().map(|_| {
            println!("Saving result image...");
            match save_buffer_to_bmp(&float_buffer, WIDTH as u32, HEIGHT as u32, "result.bmp") {
                Ok(_) => { println!("Saved."); },
                Err(message) => { println!("[ERROR]: {}", message); }
            }
        });

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

                // Sample rays
                let samples: u32 = 32;
                let single_color_contribution: f32 = 1.0 / samples as f32;

                // Initialize the pixel color
                let mut pixel_color: Vector3D = Vector3D::new_as_zero();


                let mut num_no_bounce: u32 = 0;
                let max_num_no_bounce: u32 = if samples / 4 >= 2 { samples / 4 } else { 2 };

                let mut s: u32 = 0;
                while s < samples {
                    // Ray random vibration
                    let small_pixel_offset_x = (2.0 * rng.gen::<f32>() - 1.0) * (0.5 / WIDTH as f32);
                    let small_pixel_offset_y = (2.0 * rng.gen::<f32>() - 1.0) * (0.5 / HEIGHT as f32);
                    let sample_film_plane_point = vec_sum_components(
                        &film_plane_point,
                        small_pixel_offset_x, small_pixel_offset_y, 0.0
                    );

                    // Ray
                    let direction = vec_normalize(&vec_sub(&sample_film_plane_point, &camera_pos));
                    let line: Line = Line::new(camera_pos, direction);

                    let (trace_color, bounces) = trace(&world, &line, 32);

                    num_rays += bounces as u64;
                    if bounces == 0 {
                        num_no_bounce += 1;
                    } else {
                        num_no_bounce = 0;
                    }

                    // Add the result color to the pixel's final color.
                    pixel_color = vec_sum(&pixel_color, &vec_multiplication(&trace_color, single_color_contribution));

                    s += 1;

                    // Break in case we have no bounces in this pixel.
                    if num_no_bounce >= max_num_no_bounce {
                        pixel_color = trace_color;
                        break;
                    }
                }

                let index = j * WIDTH + i;

                let gamma_corrected_color = linear_color_to_sRGB(&pixel_color);

                buffer[index] = color_to_u32(&gamma_corrected_color);
                float_buffer[index] = gamma_corrected_color;
            }

            j += 1;
            
        } else if !finised {
            finised = true;

            println!("Number of rays: {}", num_rays);
            println!("Time elapsed: {}ms", time.elapsed().as_millis());
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}

 