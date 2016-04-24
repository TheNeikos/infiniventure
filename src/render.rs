
use piston_window::*;
use gfx;
use gfx::traits::FactoryExt;
use gfx::traits::Factory;
use gfx_device_gl;
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use vecmath::{col_mat4_mul, mat4_id};

use state::State;
use geo::*;

gfx_vertex_struct!( Vertex {
    a_pos: [i8; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
    a_face: u8 = "a_face",
});

impl Vertex {
    pub fn new(pos: [i8; 3], tc: [i8; 2], face: u8) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1],
            a_tex_coord: tc,
            a_face: face
        }
    }
}

#[allow(dead_code)]
gfx_pipeline!( cube_pipeline {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    u_faces: gfx::Global<[i32; 3]> = "u_faces",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    u_height: gfx::Global<f32> = "u_height",
    u_width: gfx::Global<f32> = "u_width",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct PSOBuffer {
    cube: gfx::PipelineState<gfx_device_gl::Resources, cube_pipeline::Meta>,
}

impl PSOBuffer {
    pub fn new<U, W: Window>(window: &mut PistonWindow<U, W>) -> PSOBuffer {
        PSOBuffer {
            cube: window.factory.create_pipeline_simple(
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/cube_150.glslv"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/cube_150.glslf"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      gfx::state::CullFace::Nothing,
                      cube_pipeline::new()
                      ).unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum VertexBuffer {
    Cube
}

pub trait Renderable {
    /// We assume that it has been checked beforehand that it is drawing time
    fn draw<W: Window>(&self, w: &mut PistonWindow<State, W>, proj: [[f32; 4]; 4]);

    fn get_translation(&self) -> [[f32; 4]; 4];
}

impl Renderable for Cube {
    fn draw<W: Window>(&self, window: &mut PistonWindow<State, W>, proj: [[f32; 4]; 4]) {
        let &(ref vbuf, ref slice) = window.app.get_buffer(VertexBuffer::Cube).unwrap();
        let tex = window.app.get_texture("sprite.png").unwrap();
        let pso = &window.app.get_psos().cube;

        let proj = col_mat4_mul(proj, self.get_translation());

        let data = cube_pipeline::Data {
            vbuf: vbuf.clone(),
            u_model_view_proj: proj,
            t_color: (tex.view.clone(), window.factory.create_sampler(
                    gfx::tex::SamplerInfo::new(
                        gfx::tex::FilterMethod::Scale,
                        gfx::tex::WrapMode::Clamp
                        ))),
            u_width: tex.get_size().0 as f32,
            u_height: tex.get_size().1 as f32,
            u_faces: self.faces.to_array(),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };
        window.encoder.draw(&slice, &pso, &data);
    }

    fn get_translation(&self) -> [[f32; 4]; 4] {
        let mut id = mat4_id();
        let pos = self.shape.half_extents();
        id[3][0] = pos.x;
        id[3][1] = pos.y;
        id[3][2] = pos.z;
        id
    }
}


