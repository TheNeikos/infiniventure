extern crate piston_window;
extern crate camera_controllers;
#[macro_use] extern crate gfx;

use piston_window::*;

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
    let mut window : PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut events : WindowEvents = window.events();

    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |_c, g| {
            clear([0.3, 0.85, 0.2, 1.0], g); // A lovely green
        });
    }
}
