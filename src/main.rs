extern crate piston_window;
extern crate camera_controllers;
#[macro_use] extern crate gfx;
extern crate gfx_device_gl;
extern crate shader_version;
extern crate vecmath;
extern crate sdl2_window;
extern crate ncollide;
extern crate nalgebra as na;

use piston_window::*;
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

    let mut cubes = Vec::new();

    for x in 0..100 {
        for y in 0..100 {
            let height = ((x as f32 + y as f32) * 0.05).sin().abs() * 5.0;
            let kind = match height {
                _ if height > 2.5  => geo::CubeType::Grass,
                _ if height < 2.5  => geo::CubeType::Stone,
                _ => geo::CubeType::Grass,
            };
            cubes.push(geo::Cube::new(Vector3::new(x as f32, height.floor(), y as f32),
                                      Vector3::new(1.0, 1.0, 1.0), kind));
        }
    }

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

            for cube in cubes.iter() {
                cube.draw(&mut window, proj);
            }

            window.encoder.flush(&mut window.device);
        };

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
        }
    }
}
