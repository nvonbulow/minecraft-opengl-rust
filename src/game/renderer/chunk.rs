
use super::{Vertex, Normal, Drawable, GameRenderer};
use ::game::GameState;
use ::game::world::Chunk;
use glium::{self, Surface};
use std;

enum Face {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back
}

pub struct ChunkRenderer;

// http://bytebash.com/2012/03/opengl-volume-rendering/
impl ChunkRenderer {

    fn face_for_direction(x: u8, y: u8, z: u8, direction: Face) -> ([Vertex; 6], [Normal; 6]) {
        let (x, y, z) = (x as f32, y as f32, z as f32);
        // Top back left clockwise and then bottom back left clockwise
        let v = [
            Vertex { position: (x + 0.0, y + 1.0, z + 1.0) },
            Vertex { position: (x + 1.0, y + 1.0, z + 1.0) },
            Vertex { position: (x + 1.0, y + 1.0, z + 0.0) },
            Vertex { position: (x + 0.0, y + 1.0, z + 0.0) },

            Vertex { position: (x + 0.0, y + 0.0, z + 1.0) },
            Vertex { position: (x + 1.0, y + 0.0, z + 1.0) },
            Vertex { position: (x + 1.0, y + 0.0, z + 0.0) },
            Vertex { position: (x + 0.0, y + 0.0, z + 0.0) }
        ];
        let n = [
            Normal { normal: (-1.0, 1.0, 1.0) },
            Normal { normal: (1.0, 1.0, 1.0) },
            Normal { normal: (1.0, 1.0, -1.0) },
            Normal { normal: (-1.0, 1.0, -1.0) },

            Normal { normal: (-1.0, -1.0, 1.0) },
            Normal { normal: (1.0, -1.0, 1.0) },
            Normal { normal: (1.0, -1.0, -1.0) },
            Normal { normal: (-1.0, -1.0, -1.0) }
        ];
        match direction {
            Face::Left => {
                ([
                    v[0], v[3], v[7],
                    v[3], v[4], v[7]
                ],[
                    n[0], n[3], n[7],
                    n[3], n[4], n[7]
                ])
            },
            Face::Right => {
                ([
                    v[1], v[2], v[5],
                    v[2], v[5], v[6]
                ],[
                    n[1], n[2], n[5],
                    n[2], n[5], n[6]
                ])
            },
            Face::Top => {
                ([
                    v[0], v[1], v[2],
                    v[0], v[2], v[3]
                ],[
                    n[0], n[1], n[2],
                    n[0], n[2], n[3]
                ])
            },
            Face::Bottom => {
                ([
                    v[4], v[5], v[6],
                    v[5], v[6], v[7]
                 ],[
                    n[4], n[5], n[6],
                    n[5], n[6], n[7]
                ])
            },
            Face::Front => {
                ([
                    v[2], v[3], v[6],
                    v[2], v[3], v[7]
                 ],[
                    n[2], n[3], n[6],
                    n[2], n[3], n[7]
                ])
            },
            Face::Back => {
                ([
                    v[0], v[1], v[4],
                    v[0], v[1], v[5]
                ],[
                    n[0], n[1], n[4],
                    n[0], n[1], n[5]
                ])
            }
        }
    }

    fn generate_mesh(chunk: &Chunk) -> (Vec<Vertex>, Vec<Normal>) {
        let mut verts: Vec<Vertex> = Vec::new();
        let mut norms: Vec<Normal> = Vec::new();

        for(coordinate, block) in &chunk.blocks {
            let &(x, y, z) = coordinate;

            let mut push_face = |face: Face| {
                let (v, n) = Self::face_for_direction(x, y, z, face);
                verts.extend_from_slice(&v);
                norms.extend_from_slice(&n);
            };

            if x <= 0 {
                push_face(Face::Left);
            }
            else if !chunk.block_at(x - 1, y, z).is_solid() {
                push_face(Face::Left);
            }

            if x >= 15 {
                push_face(Face::Right);
            }
            else if !chunk.block_at(x + 1, y, z).is_solid() {
                push_face(Face::Right);
            }

            if z <= 0 {
                push_face(Face::Front);
            }
            else if !chunk.block_at(x, y, z - 1).is_solid() {
                push_face(Face::Front);
            }

            if z >= 15 {
                push_face(Face::Back);
            }
            else if !chunk.block_at(x, y, z + 1).is_solid() {
                push_face(Face::Back);
            }

            if y <= 0 {
                push_face(Face::Bottom);
            }
            else if !chunk.block_at(x, y - 1, z).is_solid() {
                push_face(Face::Bottom);
            }

            if y >= 255 {
                push_face(Face::Top);
            }
            else if !chunk.block_at(x, y + 1, z).is_solid() {
                push_face(Face::Top);
            }
        }

        (verts, norms)
    }

    pub fn draw(chunk: &Chunk, frame: &mut glium::Frame, renderer: &mut GameRenderer, state: &GameState) {
        let (positions, normals) = Self::generate_mesh(chunk);

        let vbuf_vertex = glium::VertexBuffer::new(renderer.display, &positions).unwrap();
        let vbuf_normal = glium::VertexBuffer::new(renderer.display, &normals).unwrap();

        let shader = renderer.shaders.get_shader("test");

        let (x, z) = chunk.coordinates();

        let model = [
            [1.0, 0.0, 0.0, x as f32 * 16.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, z as f32 * 16.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let uniforms = uniform! {
            perspective: state.camera.get_perspective(),
            view: state.camera.get_view(),
            model: model,
            u_light: [8.0, 8.0, 25.0f32]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            .. Default::default()
        };

        frame.draw((&vbuf_vertex, &vbuf_normal), glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), shader, &uniforms, &params).unwrap();
    }
}
