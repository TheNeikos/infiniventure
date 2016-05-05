
use piston_window::*;
use gfx;
use gfx::traits::FactoryExt;
use gfx::traits::Factory;
use gfx::handle::DepthStencilView;
use gfx::handle::ShaderResourceView;
use gfx::handle::RenderTargetView;
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

gfx_vertex_struct!( ScreenVertex {
    pos: [f32; 4] = "a_pos",
});

impl ScreenVertex {
    pub fn new(pos: [f32; 2]) -> ScreenVertex {
        ScreenVertex {
            pos: [pos[0], pos[1], 0.0, 1.0],
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

gfx_constant_struct!( SSAOParams {
    sample_rad: f32 = "sample_rad",
});

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

gfx_pipeline!( ssao_pipeline {
    vbuf: gfx::VertexBuffer<ScreenVertex> = (),
    in_depth: gfx::TextureSampler<f32> = "in_depth",
    in_color: gfx::TextureSampler<[f32; 4]> = "in_color",
    enabled: gfx::Global<i32> = "enabled",
    ssao_constants: gfx::ConstantBuffer<SSAOParams> = "SSAOParams",
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_color",
});

gfx_pipeline!( blur_pipeline {
    vbuf: gfx::VertexBuffer<ScreenVertex> = (),
    horizontal: gfx::Global<i32> = "u_horizontal",
    in_color: gfx::TextureSampler<[f32; 4]> = "in_color",
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_color",
});

pub struct PSOBuffer {
    pub cube: gfx::PipelineState<gfx_device_gl::Resources, cube_pipeline::Meta>,
    pub ssao: gfx::PipelineState<gfx_device_gl::Resources, ssao_pipeline::Meta>,
    pub blur: gfx::PipelineState<gfx_device_gl::Resources, blur_pipeline::Meta>,
}

impl PSOBuffer {
    pub fn new<W: Window>(window: &mut PistonWindow<W>) -> PSOBuffer {
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
            blur: window.factory.create_pipeline_simple(
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/blur_150.glslv"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/blur_150.glslf"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      gfx::state::CullFace::Nothing,
                      blur_pipeline::new()
                      ).unwrap(),
            ssao: window.factory.create_pipeline_simple(
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/screen_150.glslv"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      Shaders::new().set(GLSL::V1_50,
                                         include_str!("../asset_src/ssao_150.glslf"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
                      gfx::state::CullFace::Nothing,
                      ssao_pipeline::new()
                      ).unwrap(),
        }
    }
}

pub struct RTBuffer {
    pub depth_r: ShaderResourceView<gfx_device_gl::Resources, f32>,
    pub depth_w: DepthStencilView<gfx_device_gl::Resources, gfx::format::DepthStencil>,
    pub render_r: ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    pub render_w: RenderTargetView<gfx_device_gl::Resources, gfx::format::Srgba8>,
    pub render2_r: ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>,
    pub render2_w: RenderTargetView<gfx_device_gl::Resources, gfx::format::Srgba8>,
}

impl RTBuffer {
    pub fn new<W: Window>(w: &mut PistonWindow<W>) -> RTBuffer {
        let draw_size = w.draw_size();
        let (depr, depw) = {
            let kind = gfx::tex::Kind::D2(draw_size.width as u16, draw_size.height as u16, gfx::tex::AaMode::Single);
            let bind = gfx::SHADER_RESOURCE | gfx::DEPTH_STENCIL;
            let cty = gfx::format::ChannelType::Unorm;
            let tex = w.factory.create_texture(kind, 1, bind, gfx::Usage::GpuOnly, Some(cty)).unwrap();
            let depr = w.factory.view_texture_as_shader_resource::<gfx::format::DepthStencil>(&tex, (0, 0), gfx::format::Swizzle::new()).unwrap();
            let depw = w.factory
                .view_texture_as_depth_stencil::<gfx::format::DepthStencil>(&tex, 0, None, gfx::tex::DepthStencilFlags::empty()).unwrap();
            (depr, depw)
        };

        let (renr, renw) = {
            let kind = gfx::tex::Kind::D2(draw_size.width as u16, draw_size.height as u16, gfx::tex::AaMode::Single);
            let bind = gfx::SHADER_RESOURCE | gfx::RENDER_TARGET ;
            let cty = gfx::format::ChannelType::Srgb;
            let tex = w.factory.create_texture(kind, 1, bind, gfx::Usage::GpuOnly, Some(cty)).unwrap();
            let r = w.factory.view_texture_as_shader_resource::<gfx::format::Srgba8>(&tex, (0, 0), gfx::format::Swizzle::new()).unwrap();
            let w = w.factory.view_texture_as_render_target::<gfx::format::Srgba8>(&tex, 0, None).unwrap();
            (r, w)
        };

        let (renr2, renw2) = {
            let kind = gfx::tex::Kind::D2(draw_size.width as u16, draw_size.height as u16, gfx::tex::AaMode::Single);
            let bind = gfx::SHADER_RESOURCE | gfx::RENDER_TARGET ;
            let cty = gfx::format::ChannelType::Srgb;
            let tex = w.factory.create_texture(kind, 1, bind, gfx::Usage::GpuOnly, Some(cty)).unwrap();
            let r = w.factory.view_texture_as_shader_resource::<gfx::format::Srgba8>(&tex, (0, 0), gfx::format::Swizzle::new()).unwrap();
            let w = w.factory.view_texture_as_render_target::<gfx::format::Srgba8>(&tex, 0, None).unwrap();
            (r, w)
        };

        RTBuffer {
            depth_r: depr,
            depth_w: depw,
            render_r: renr,
            render_w: renw,
            render2_r: renr2,
            render2_w: renw2,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum VertexBuffer {
    Cube, Screen
}

pub trait Renderable {
    /// We assume that it has been checked beforehand that it is drawing time
    fn draw<W: Window>(&self, w: &mut PistonWindow<W>, proj: [[f32; 4]; 4]);
}

pub trait Instanceable {
    type InstanceType : Sized;
    fn instance<W: Window>(&self, w: &mut PistonWindow<W>) -> Self::InstanceType;
}

impl Instanceable for Cube {
    type InstanceType = CubeInstance;
    fn instance<W: Window>(&self, w: &mut PistonWindow<W>) -> Self::InstanceType {
        CubeInstance::new(self.pos(), self.faces.to_array())
    }
}

