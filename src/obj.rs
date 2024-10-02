use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
}

impl Obj {
    // Load the OBJ file and extract vertices, normals, texcoords, and indices
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        // Load the OBJ file, triangulate the model by passing `true`
        let (models, _) = tobj::load_obj(filename, true)?;

        // Assume we're working with the first model in the file (index 0)
        let mesh = &models[0].mesh;

        // Extract vertices, normals, texcoords, and indices
        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        // Return the parsed data in the Obj struct
        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    // Generate an array of Vertex structs using the vertex, normal, and texture coordinate indices
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();

        for i in 0..self.indices.len() {
            let index = self.indices[i] as usize;

            let position = self.vertices.get(index)
                .expect("Invalid vertex index in .obj file");

            let default_normal = Vec3::new(0.0, 0.0, 0.0);
            let normal = if !self.normals.is_empty() {
                self.normals.get(index).unwrap_or(&default_normal)
            } else {
                &default_normal // Default normal if none are present
            };

            let default_tex_coords = Vec2::new(0.0, 0.0);
            let tex_coords = if !self.texcoords.is_empty() {
                self.texcoords.get(index).unwrap_or(&default_tex_coords)
            } else {
                &default_tex_coords // Default UV coordinates if none are present
            };

            let vertex = Vertex::new(*position, *normal, *tex_coords);
            vertex_array.push(vertex);
        }

        vertex_array
    }
}
