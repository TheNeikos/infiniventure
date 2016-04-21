extern crate piston_window;
extern crate camera_controllers;
#[macro_use] extern crate gfx;
extern crate gfx_device_gl;
extern crate shader_version;
extern crate vecmath;
extern crate sdl2_window;
extern crate ncollide;
extern crate nalgebra as na;

use std::path::Path;

use piston_window::*;
use gfx::traits::*;
use sdl2_window::Sdl2Window;
use na::Vector3;

use camera_controllers::{FirstPersonSettings, FirstPerson, CameraPerspective,
                        model_view_projection};

mod geo;
mod render;
mod state;

use render::Renderable;

fn main() {
    let mut window : PistonWindow<(), Sdl2Window> = WindowSettings::new("Rustcraft!", [640, 480])
        .samples(4).exit_on_esc(true).build().unwrap();
    window.set_capture_cursor(true);

    let state = state::State::new(&mut window);

    let mut window = window.app(state);

    let get_projection = |w: &PistonWindow<state::State, Sdl2Window>| {
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

    first_person.velocity = 10.0;

    let cube = geo::Cube::new(Vector3::new(0.0, 0.0, 0.0), geo::CubeType::Stone);
    let cube2 = geo::Cube::new(Vector3::new(2.0, 0.0, 0.0), geo::CubeType::Grass);

    while let Some(e) = window.next() {
        first_person.event(&e);

        if let Some(args) = e.render_args() {
            window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            let proj = model_view_projection(
                model_proj,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );

            cube.draw(&mut window, proj);
            cube2.draw(&mut window, proj);

            window.encoder.flush(&mut window.device);
        };

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
        }
    }
}
