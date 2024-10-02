mod color;
mod framebuffer;
mod vertex;
mod fragment;
mod obj;
mod line;
mod pipeline;  // New pipeline module

use crate::pipeline::{render, Uniforms};
use crate::obj::Obj;
use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

fn main() {
    // Declare framebuffer as mutable since we need to modify it in the render function
    let mut framebuffer = Framebuffer::new(800, 600);

    // Load the OBJ model and generate vertices
    let obj = Obj::load("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/Renderer/src/model/Ragdoll.obj").unwrap();
    let vertex_array = obj.get_vertex_array();

    // Create uniforms with identity matrices for now (you can set these to real transformations later)
    let uniforms = Uniforms {
        model_matrix: nalgebra_glm::Mat4::identity(),
        view_matrix: nalgebra_glm::Mat4::identity(),
        projection_matrix: nalgebra_glm::Mat4::identity(),
    };

    // Render the object
    render(&mut framebuffer, &uniforms, &vertex_array);
}
