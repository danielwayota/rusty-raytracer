use crate::geometry::{
    Mesh, Triangle
};

use crate::vector3d::{Vector3D};

pub fn load_obj(obj_file: String, material_index: usize) -> Mesh {
    let mut mesh: Mesh = Mesh::new();

    let mut vertices: Vec<Vector3D> = Vec::new();

    for line in obj_file.split("\n") {
        let components: Vec<&str> = line.split(" ").collect();

        match components[0] {
            "v" => {
                // v 1.000000 1.000000 -1.000000
                let x: f32 = components[1].parse::<f32>().unwrap();
                let y: f32 = components[2].parse::<f32>().unwrap();
                let z: f32 = components[3].parse::<f32>().unwrap();

                vertices.push(Vector3D::new(x, y, z));
            },
            "f" => {
                // This is a face/triangle
                // f 1/1/1 5/2/1 7/3/1 3/4/1
                let mut triangle_vertices: Vec<Vector3D> = Vec::new();

                for i in 1..components.len() {
                    let indices: Vec<&str> = components[i].split("/").collect();

                    // FIXME: Use the texture information
                    let face_index: usize = indices[0].parse::<usize>().unwrap() - 1;

                    triangle_vertices.push(vertices[face_index]);
                }

                mesh.triangles.push(
                    Triangle::new(
                        triangle_vertices[0],
                        triangle_vertices[2],
                        triangle_vertices[1],
                        material_index
                    )
                );
            },
            _ => {}
        }
    }

    return mesh;
}