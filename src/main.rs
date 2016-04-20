extern crate piston_window;
extern crate camera_controllers;
#[macro_use] extern crate gfx;
extern crate shader_version;
extern crate vecmath;
extern crate sdl2_window;

use std::path::Path;

use piston_window::*;
use gfx::traits::*;
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use sdl2_window::Sdl2Window;

use camera_controllers::{FirstPersonSettings, FirstPerson, CameraPerspective,
                        model_view_projection};

gfx_vertex_struct!( Vertex {
    a_pos: [i8; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
});

impl Vertex {
    fn new(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1],
            a_tex_coord: tc,
        }
    }
}

// Taken from piston-examples
gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

fn main() {
    let mut window : PistonWindow<(), Sdl2Window> = WindowSettings::new("Hello Piston!", [640, 480])
        .samples(4).exit_on_esc(true).build().unwrap();
    window.set_capture_cursor(true);

    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([-1, -1,  1], [0, 0]),
        Vertex::new([ 1, -1,  1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //bottom (0, 0, -1)
        Vertex::new([ 1,  1, -1], [0, 0]),
        Vertex::new([-1,  1, -1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
        //right (1, 0, 0)
        Vertex::new([ 1, -1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([ 1, -1,  1], [0, 1]),
        //left (-1, 0, 0)
        Vertex::new([-1,  1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([-1,  1, -1], [0, 1]),
        //front (0, 1, 0)
        Vertex::new([-1,  1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //back (0, -1, 0)
        Vertex::new([ 1, -1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
        ];

    let index_data: &[u8] = &[
        0,  1,  2,  2,  3,  0, // top
        4,  6,  5,  6,  4,  7, // bottom
        8,  9, 10, 10, 11,  8, // right
        12, 14, 13, 14, 12, 16, // left
        16, 18, 17, 18, 16, 19, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let (vbuf, slice) = window.factory.create_vertex_buffer_indexed(&vertex_data,
                                                                    index_data);

    let stone = Texture::from_path(
        &mut window.factory,
        &Path::new("assets/stone.png"),
        Flip::None,
        &TextureSettings::new(),
        ).unwrap();


    let sampler_info = gfx::tex::SamplerInfo::new(
        gfx::tex::FilterMethod::Scale,
        gfx::tex::WrapMode::Clamp
        );

    let pso = window.factory.create_pipeline_simple(
        Shaders::new().set(GLSL::V1_50,
                           include_str!("../asset_src/cube_150.glslv"))
                       .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
        Shaders::new().set(GLSL::V1_50,
                           include_str!("../asset_src/cube_150.glslf"))
                      .get(OpenGL::V3_2.to_glsl()).unwrap().as_bytes(),
        gfx::state::CullFace::Nothing,
        pipe::new()
        ).unwrap();

    let get_projection = |w: &PistonWindow<(), Sdl2Window>| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: draw_size.width as f32 / draw_size.height as f32
        }.projection()
    };

    let model_proj = vecmath::mat4_id();
    let mut projection = get_projection(&window);

    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let mut data = pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        t_color: (stone.view, window.factory.create_sampler(sampler_info)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
        };


    while let Some(e) = window.next() {
        first_person.event(&e);

        if let Some(_) = e.render_args() {
            let args = e.render_args().unwrap(); // We can unwrap as this closure only gets called
                                                 // in the render loop
            window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model_proj,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            window.encoder.draw(&slice, &pso, &data);
            window.encoder.flush(&mut window.device);
        };

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
        }
    }
}
