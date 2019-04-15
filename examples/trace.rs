use std::io::Result as IOResult;
use std::time::Instant;

extern crate bmp;
use bmp::{Image, Pixel};

extern crate minifb;
use minifb::{
    Window, WindowOptions,
    Menu
};

extern crate rand;
use rand::prelude::*;

extern crate raytracer;
use raytracer::{
    trace, World, PointLight
};

use raytracer::geometry::{
    Line, Sphere, Triangle
};

use raytracer::vector3d::{
    Vector3D,
    vec_sum, vec_sum_components, vec_sub, vec_multiplication,
    vec_normalize
};
use raytracer::color::{
    Material, color_to_u32,
    float_color_from_bytes, linear_color_to_srgb
};
use raytracer::camera::{Camera};


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

    // --- Window setup ---

    // The integer buffer is used to display the image on screen
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // The float buffer is needed to save the final image as BPM.
    let mut float_buffer: Vec<Vector3D> = vec![Vector3D::new_as_zero(); WIDTH * HEIGHT];

    let mut window = Window::new("Raycaster - The Begining", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // --- Raycast stuff ---

    // Le camera
    let camera: Camera = Camera::new(
        Vector3D::new(0.0, 8.0, 8.0),
        Vector3D::new(0.0, 0.0, 0.0),
        2.0
    );

    // --- World objects ---
    let mut world = World::new();

    // Save result image Menu
    let mut save_menu = Menu::new("File").unwrap();
    save_menu.add_item("Save Image", 42).build();
    window.add_menu(&save_menu);

    // Random
    let mut rng = rand::thread_rng();

    world.materials = vec![
        // Sky color and ambient light
        Material::new_light(
            // Vector3D::new(0.25, 0.8, 0.9)
            // Vector3D::new(0.2, 0.2, 0.25)
            Vector3D::new(0.1, 0.1, 0.15)
            // Vector3D::new_as_zero()
        ),
        // Floor
        Material::new(
            float_color_from_bytes(88, 117, 167),
            Vector3D::new_as_zero(),
            0.8,
            0.0
        ),
        
        //Spheres
        Material::new(
            float_color_from_bytes(250, 250, 250),
            Vector3D::new_as_zero(),
            0.05,
            0.9
        ),
        Material::new(
            float_color_from_bytes(212, 175, 55),
            Vector3D::new_as_zero(),
            0.05,
            0.2
        ),
        // Lights
        Material::new_light(Vector3D::new(5.0, 5.0, 5.0)),
        Material::new_light(Vector3D::new(5.0, 0.25, 0.25))
    ];

    world.materials.push(
        Material::new_light(Vector3D::new(0.8, 1.0, 0.8))
    );

    world.lights.push(PointLight::new( Vector3D::new(2.0, 5.0, 0.0), Vector3D::new(0.9, 0.9, 0.9), 5.0 ));
    world.lights.push(PointLight::new( Vector3D::new(-2.0, 5.0, 0.0), Vector3D::new(0.9, 0.7, 0.9), 5.0 ));

    world.objects.push(
        Box::new(
            Triangle::new(
                Vector3D::new( 0.0, 0.0, -5.0),
                Vector3D::new( 5.0, 0.0, 5.0),
                Vector3D::new(-5.0, 0.0, 5.0),
                1
            )
        )
    );

    // world.lights.push(PointLight::new( Vector3D::new(-2.0, 5.0, 0.0), Vector3D::new(0.2, 0.1, 0.1) ));

    let size: isize = 1;

    for sx in -size..size {
        for sy in -size..size {
            world.objects.push(
                Box::new(
                    Sphere::new(
                        Vector3D::new(
                            sx as f32 * 2.0 + 0.5,
                            3.0,
                            sy as f32 * 2.0 + 0.5
                        ),0.25, 4
                    )
                )
            );
        }
    }
    let size: isize = 2;

    for sx in -size..size {
        for sy in -size..size {
            world.objects.push(
                Box::new(
                    Sphere::new(
                        Vector3D::new(
                            sx as f32 + 0.5,
                            1.0,
                            sy as f32 + 0.5
                        ), 0.5, if (sx + sy) % 2 != 0 {3} else {2}
                    )
                )
            );
        }
    }

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
            // Horizontal scan line
            for i in 0..WIDTH {
                let film_plane_point = camera.screen_point_to_projection_plane(i, WIDTH, j, HEIGHT);

                // Sample rays
                let samples: u32 = 2;
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
                    let direction = vec_normalize(&vec_sub(&sample_film_plane_point, &camera.position));
                    let line: Line = Line::new(camera.position, direction);

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

                let gamma_corrected_color = linear_color_to_srgb(&pixel_color);

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
