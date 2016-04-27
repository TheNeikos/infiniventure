
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
    pos: [f32; 4] = "a_pos",
    tex_coord: [i8; 2] = "a_tex_coord",
    face: u8 = "a_face",
});

impl Vertex {
    pub fn new(pos: [f32; 3], tc: [i8; 2], face: u8) -> Vertex {
        Vertex {
            pos: [pos[0], pos[1], pos[2], 1.0],
            tex_coord: tc,
            face: face,
        }
    }
}

gfx_vertex_struct!( CubeInstance {
    pos: [f32; 3] = "a_translate",
    faces: [i32; 3] = "a_faces",
});

impl CubeInstance {
    pub fn new(pos: [f32; 3], faces: [i32; 3]) -> CubeInstance {
        CubeInstance {
            pos: pos,
            faces: faces,
        }
    }
}

#[allow(dead_code)]
gfx_pipeline!( cube_pipeline {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    instances: gfx::InstanceBuffer<CubeInstance> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    u_height: gfx::Global<f32> = "u_height",
    u_width: gfx::Global<f32> = "u_width",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct PSOBuffer {
    pub cube: gfx::PipelineState<gfx_device_gl::Resources, cube_pipeline::Meta>,
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
                      gfx::state::CullFace::Back,
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
}

pub trait Instanceable {
    type InstanceType : Sized;
    fn instance<W: Window>(&self, w: &mut PistonWindow<State, W>) -> Self::InstanceType;
}

impl Instanceable for Cube {
    type InstanceType = CubeInstance;
    fn instance<W: Window>(&self, w: &mut PistonWindow<State, W>) -> Self::InstanceType {
        CubeInstance::new(self.pos(), self.faces.to_array())
    }
}

