use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use nalgebra_glm::{Mat4, Vec3};



pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Convert position to homogeneous coordinates (Vec4)
    let pos_vec4 = vertex.position.push(1.0);

    // Apply model-view-projection transformation
    let transformed_position = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * pos_vec4;

    // Convert back to Vec3 (perspective divide by w)
    let transformed_position_vec3 = transformed_position.xyz() / transformed_position.w;

    // Create the transformed vertex
    let mut transformed_vertex = vertex.clone();
    transformed_vertex.transformed_position = transformed_position_vec3;

    transformed_vertex
}


fn assemble_primitives(vertex_array: &[Vertex]) -> Vec<(Vertex, Vertex, Vertex)> {
    // Assuming the vertices are provided in a sequence where each three vertices form a triangle
    let mut triangles = Vec::new();

    for chunk in vertex_array.chunks(3) {
        if let [v1, v2, v3] = chunk {
            triangles.push((v1.clone(), v2.clone(), v3.clone()));
        }
    }

    triangles
}

// Simple placeholder rasterization logic to generate fragments from triangles
fn rasterize_triangle(triangle: (Vertex, Vertex, Vertex)) -> Vec<Fragment> {
    let (v1, v2, v3) = triangle;

    // Call the line drawing function from earlier
    let mut fragments = Vec::new();
    fragments.extend(crate::line::line(&v1, &v2));
    fragments.extend(crate::line::line(&v2, &v3));
    fragments.extend(crate::line::line(&v3, &v1));

    fragments
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage: Transform vertices using the vertex shader
    let transformed_vertices: Vec<Vertex> = vertex_array.iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();

    // Primitive Assembly Stage: Assemble vertices into triangles
    let triangles = assemble_primitives(&transformed_vertices);

    // Rasterization: Convert triangles into fragments
    for triangle in triangles {
        let fragments = rasterize_triangle(triangle);

        // Fragment Processing Stage: Write fragments to the framebuffer
        for fragment in fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;

            if x < framebuffer.width && y < framebuffer.height {
                let color = fragment.color.to_hex();
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}
