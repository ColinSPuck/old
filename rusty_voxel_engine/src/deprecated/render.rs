// render.rs

use glium::{self, index::{NoIndices, PrimitiveType}, Display, VertexBuffer, Program, implement_vertex, Surface, uniform};
use crate::octree_builder::{SparseVoxelOctree, Node};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32, f32),
    color: (u8, u8, u8),
}

implement_vertex!(Vertex, position, color);

pub struct Renderer {
    display: Display,
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
}

impl Renderer {
    pub fn new(display: Display, octree: &SparseVoxelOctree) -> Self {
        let mut vertices = Vec::new();
        Self::collect_leaf_voxels(&octree.root, &mut vertices);

        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        Self {
            display,
            vertex_buffer,
            program,
        }
    }

    fn collect_leaf_voxels(node: &Option<Arc<RwLock<Node>>>, vertices: &mut Vec<Vertex>) {
        if let Some(node) = node {
            let node = node.read();
            if node.is_leaf {
                vertices.push(Vertex {
                    position: (node.data.x as f32, node.data.y as f32, node.data.z as f32),
                    color: (node.data.r, node.data.g, node.data.b),
                });
            } else {
                for child in node.children.iter() {
                    Self::collect_leaf_voxels(child, vertices);
                }
            }
        }
    }

    pub fn render(&self, model: [[f32; 4]; 4], view: [[f32; 4]; 4], projection: [[f32; 4]; 4]) {
        let view = view;
        let projection = projection;
        let uniforms = uniform! {
            model: model,
            view: view,
            projection: projection,
        };

        let params = glium::DrawParameters {
            point_size: Some(5.0),
            ..Default::default()
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&self.vertex_buffer, NoIndices(PrimitiveType::Points), &self.program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }
    
}

static VERTEX_SHADER_SRC: &str = r#"
    #version 330 core

    layout(location = 0) in vec3 position;
    layout(location = 1) in vec3 color;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 v_color;

    void main() {
        vec4 world_pos = model * vec4(position, 1.0);
        gl_Position = projection * view * world_pos;
        v_color = color / 255.0;
    }
"#;

static FRAGMENT_SHADER_SRC: &str = r#"
    #version 330 core

    in vec3 v_color;
    out vec4 FragColor;

    void main() {
        FragColor = vec4(v_color, 1.0);
    }
"#;
