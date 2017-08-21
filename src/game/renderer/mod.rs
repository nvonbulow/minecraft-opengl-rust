use glium;
use glium::Surface;

use ::game::GameState;

pub mod chunk;
pub mod shaders;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32)
}

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32)
}

pub struct GameRenderer<'a> {
    display: &'a glium::Display,
    pub shaders: shaders::ShaderCache<'a>
}

impl<'a> GameRenderer<'a> {
    pub fn new(display: &'a glium::Display) -> GameRenderer<'a> {
        implement_vertex!(Vertex, position);
        implement_vertex!(Normal, normal);

        GameRenderer {
            display,
            shaders: shaders::ShaderCache::with_display(display)
        }
    }

    pub fn render_frame(&mut self, frame: &mut glium::Frame, state: &mut GameState) {
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let light = [0.0, 1.0, 0.0f32];

        let uniforms = uniform! {
            perspective: state.camera.get_perspective(),
            view: state.camera.get_view(),
            model: model,
            u_light:light
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };


        let positions = glium::VertexBuffer::new(self.display, &[
            Vertex { position: (0.0, 0.5, 0.0) },
            Vertex { position: (-0.5, -0.5, 0.5) },
            Vertex { position: (0.5, -0.5, 0.5) },
            Vertex { position: (0.5, -0.5, -0.5) },
            Vertex { position: (-0.5, -0.5, -0.5) }
        ]).unwrap();

        let normals = glium::VertexBuffer::new(self.display, &[
            Normal { normal: (0.0, 0.5, 0.0) },
            Normal { normal: (-1.0, 2.0, 1.0) },
            Normal { normal: (1.0, 2.0, 1.0) },
            Normal { normal: (1.0, 2.0, -1.0) },
            Normal { normal: (-1.0, 2.0, -1.0) }
        ]).unwrap();

        let indicies = glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, &[
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 1, 4,
            1, 2, 3,
            1, 3, 4u16
        ]).unwrap();

        // let test_shader = self.shaders.get_shader("test");

        let chunk = ::game::world::Chunk::new(0, 0);

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        chunk.draw(frame, self, state);
        // frame.draw((&positions, &normals), &indicies, test_shader, &uniforms, &params).unwrap();

    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut glium::Frame, renderer: &mut GameRenderer, state: &mut GameState);
}
