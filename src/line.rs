use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::{Vec2};

// Helper function to linearly interpolate between two values.
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

// Bresenham's-like line drawing algorithm to draw line between two vertices.
pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    
    // Project vertices into 2D (screen space). Assuming transformed positions are used for screen-space coordinates.
    let a_pos: Vec2 = Vec2::new(a.transformed_position.x, a.transformed_position.y);
    let b_pos: Vec2 = Vec2::new(b.transformed_position.x, b.transformed_position.y);
    
    // Get the differences between the positions
    let dx = (b_pos.x - a_pos.x).abs() as i32;
    let dy = (b_pos.y - a_pos.y).abs() as i32;
    
    // Determine the direction of the line
    let sx = if a_pos.x < b_pos.x { 1 } else { -1 };
    let sy = if a_pos.y < b_pos.y { 1 } else { -1 };
    
    let mut err = dx - dy;
    
    // Start at point `a`
    let mut current_x = a_pos.x as i32;
    let mut current_y = a_pos.y as i32;
    
    // Interpolate depth and color between the two vertices
    let total_distance = f32::sqrt(((b_pos.x - a_pos.x).powi(2) + (b_pos.y - a_pos.y).powi(2)) as f32);
    
    loop {
        // Calculate interpolation factor (t) based on the current position along the line
        let distance_covered = f32::sqrt(((current_x as f32 - a_pos.x).powi(2) + (current_y as f32 - a_pos.y).powi(2)) as f32);
        let t = distance_covered / total_distance;
        
        // Interpolate depth and color
        let depth = lerp(a.transformed_position.z, b.transformed_position.z, t);
        let color = Color::lerp(&a.color, &b.color, t);
        
        // Create a fragment at the current position
        fragments.push(Fragment {
            position: Vec2::new(current_x as f32, current_y as f32),
            color,
            depth,
        });
        
        // If we've reached the end point `b`, break
        if current_x == b_pos.x as i32 && current_y == b_pos.y as i32 {
            break;
        }
        
        // Bresenham's step
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            current_x += sx;
        }
        if e2 < dx {
            err += dx;
            current_y += sy;
        }
    }
    
    fragments
}
