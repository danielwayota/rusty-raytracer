use std::io::Result as IOResult;
use std::time::Instant;
use std::fs;

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
use raytracer::loaders::{load_obj};
use raytracer::geometry::{Line};

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

use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;

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
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    // --- Window setup ---

    // The integer buffer is used to display the image on screen
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // The float buffer is needed to save the final image as BPM.
    let mut float_buffer: Vec<Vector3D> = vec![Vector3D::new_as_zero(); WIDTH * HEIGHT];

    let mut window = Window::new("Raycaster - The Begining", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // --- Raycast stuff ---

    // Le camera
    let camera: Camera = Camera::new(
        Vector3D::new(0.0, 2.0, 6.0),
        Vector3D::new(0.0, 0.0, 0.0),
        2.0
    );

    // --- World objects ---
    let mut world = World::new();

    // Save result image Menu
    let mut save_menu = Menu::new("File").unwrap();
    save_menu.add_item("Save Image", 42).build();
    window.add_menu(&save_menu);

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

    world.lights.push(PointLight::new( Vector3D::new(2.0, 5.0, 0.0), Vector3D::new(0.9, 0.9, 0.9), 5.0 ));
    world.lights.push(PointLight::new( Vector3D::new(-2.0, 5.0, 0.0), Vector3D::new(0.9, 0.7, 0.9), 5.0 ));

    let mesh = load_obj(fs::read_to_string("./cube.obj").unwrap(), 3);

    world.objects = mesh.triangles;

    // Stats
    let mut finised = false;
    let num_rays_arc = Arc::new(Mutex::new(0));

    let time = Instant::now();

    // --------------------------------------
    // Multithread stuff
    // --------------------------------------
    let slice_count = 256;
    let buffer_length = buffer.len();
    // Get the slice size.
    let thread_buffer_length = buffer_length / slice_count;

    // (start, end)
    let mut slices: Vec<(usize, usize)> = Vec::with_capacity(slice_count);

    for index in 0..slice_count {
        // Start and end indices
        let rect_buffer_start = index * thread_buffer_length;
        let rect_buffer_end = rect_buffer_start + thread_buffer_length;

        slices.push((rect_buffer_start, rect_buffer_end));
    }

    slices.shuffle(&mut rand::thread_rng());

    // (index, colour)
    let (sender, receiver) = mpsc::channel::<Vec<(usize, Vector3D)>>();

    // Mutex and stuff conversion
    let slices_atomic_arc = Arc::new(Mutex::new(slices));
    let world_arc = Arc::new(world);

    // Create four threads
    let max_thread_count = 4;

    let thread_counter_arc = Arc::new(Mutex::from(max_thread_count));

    for thread_index in 0..max_thread_count {
        let one_sender = sender.clone();
        let local_arc = Arc::clone(&slices_atomic_arc);

        let local_world_arc = Arc::clone(&world_arc);
        let local_thread_counter_arc = Arc::clone(&thread_counter_arc);

        let local_num_rays_arc = Arc::clone(&num_rays_arc);

        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let mut local_slices = local_arc.lock().unwrap();

                if local_slices.len() == 0 {
                    break;
                }

                let slice = local_slices.pop().unwrap();
                drop(local_slices);

                let start = slice.0;
                let end = slice.1;

                let mut pixels: Vec<(usize, Vector3D)> = Vec::with_capacity(thread_buffer_length);
                for buffer_index in start..end {
                    let i = buffer_index % WIDTH;
                    let j = buffer_index / WIDTH;

                    let film_plane_point = camera.screen_point_to_projection_plane(i, WIDTH, j, HEIGHT);

                    // Sample rays
                    let samples: u32 = 8;
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

                        let (trace_color, bounces) = trace(&local_world_arc, &line, 16);

                        let mut num_rays = local_num_rays_arc.lock().unwrap();
                        *num_rays += bounces as u64;
                        drop(num_rays);

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

                    let gamma_corrected_color = linear_color_to_srgb(&pixel_color);

                    pixels.push((buffer_index, gamma_corrected_color));
                }

                one_sender.send(pixels).unwrap();
            }

            println!("Thread < {} > finised", thread_index);
            let mut counter = local_thread_counter_arc.lock().unwrap();
            *counter = *counter - 1;

            drop(counter);
        });
    }

    // --------------------------------------
    // Display the window
    // --------------------------------------
    while window.is_open() {

        window.is_menu_pressed().map(|_| {
            println!("Saving result image...");
            match save_buffer_to_bmp(&float_buffer, WIDTH as u32, HEIGHT as u32, "result.bmp") {
                Ok(_) => { println!("Saved."); },
                Err(message) => { println!("[ERROR]: {}", message); }
            }
        });

        let mut should_redraw = false;
        if !finised {
            let mut message_count = 0;

            // Retrieve the thread results.
            // Iterate over the full message list generated since the last iteration.
            for message in receiver.try_iter() {
                for value in message.iter() {
                    buffer[value.0] = color_to_u32(&value.1);
                    float_buffer[value.0] = value.1;
                }

                message_count += 1;
            }

            should_redraw = message_count != 0;

            let active_threads = thread_counter_arc.lock().unwrap();
            if *active_threads <= 0 && message_count == 0 {
                finised = true;

                let num_rays = num_rays_arc.lock().unwrap();
                println!("Number of rays: {}", num_rays);
                drop(num_rays);

                println!("Time elapsed: {}ms", time.elapsed().as_millis());
            }
            drop(active_threads);
        }

        if should_redraw {
            window.update_with_buffer(&buffer).unwrap();
        } else {
            window.update();
        }
        thread::sleep(Duration::from_millis(10));
    }
}
