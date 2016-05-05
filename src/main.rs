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
use gfx::traits::FactoryExt;
use gfx::traits::Factory;
use vecmath::{col_mat4_mul, mat4_id};

use camera_controllers::{FirstPersonSettings, FirstPerson, CameraPerspective,
                        model_view_projection};

mod geo;
mod render;
mod state;

use render::{blur_pipeline, ssao_pipeline, cube_pipeline, Renderable, VertexBuffer, Instanceable,
             SSAOParams};

fn main() {
    let mut window : PistonWindow<Sdl2Window> = WindowSettings::new("Rustcraft!", [640, 480])
        .samples(0).exit_on_esc(true).build().unwrap();
    window.set_capture_cursor(true);

    let state = state::State::new(&mut window);

    let get_projection = |w: &PistonWindow<Sdl2Window>| {
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

    let instances = cubes.iter().map(|x| x.instance(&mut window)).collect::<Vec<_>>();
    let instance_buffer = window.factory.create_buffer_dynamic(instances.len(), gfx::BufferRole::Vertex,
                                                               gfx::Bind::empty()).unwrap();

    window.encoder.update_buffer(&instance_buffer, &instances, 0).unwrap();

    let ssao_params = window.factory.create_buffer_dynamic(1, gfx::BufferRole::Uniform, gfx::Bind::empty()).unwrap();
    window.encoder.update_buffer(&ssao_params, &[SSAOParams { sample_rad: 0.5, }], 0).unwrap();

    let mut enabl = false;

    let scale_clamp_sampler = window.factory.create_sampler(
                            gfx::tex::SamplerInfo::new(
                                gfx::tex::FilterMethod::Scale,
                                gfx::tex::WrapMode::Clamp
                                ));

    while let Some(e) = window.next() {
        first_person.event(&e);

        if let Some(args) = e.render_args() {
            window.draw_3d(&e, |window| {
                window.encoder.clear(&window.output_color, [0.5, 0.0, 0.2, 1.0]);
                window.encoder.clear_depth(&window.output_stencil, 1.0);

                window.encoder.clear(&state.rts().render_w, [0.0, 0.0, 1.0, 1.0]);
                window.encoder.clear_depth(&state.rts().depth_w, 1.0);

                let proj = model_view_projection(
                    model_proj,
                    first_person.camera(args.ext_dt).orthogonal(),
                    projection
                );

                let (ref vbuf, ref slice) = state.buffers().cube;
                let tex = state.get_texture("sprite.png").unwrap();
                let pso = &state.psos().cube;

                let mut slice = slice.clone();
                slice.instances = Some((instances.len() as u32, 0));

                let data = cube_pipeline::Data {
                    vbuf: vbuf.clone(),
                    instances: instance_buffer.clone(),
                    u_model_view_proj: proj,
                    t_color: (tex.view.clone(), scale_clamp_sampler.clone()),
                    u_width: tex.get_size().0 as f32,
                    u_height: tex.get_size().1 as f32,
                    //out_color: window.output_color.clone(),
                    //out_depth: window.output_stencil.clone(),
                    out_color: state.rts().render_w.clone(),
                    out_depth: state.rts().depth_w.clone(),
                };
                window.encoder.draw(&slice, &pso, &data);

                let pso = &state.psos().blur;
                let (ref vbuf, ref slice) = state.buffers().screen;
                let data = blur_pipeline::Data {
                    vbuf: vbuf.clone(),
                    u_model_view_proj: proj,
                    horizontal: 0,
                    in_color: (state.rts().render_r.clone(), scale_clamp_sampler.clone()),
                    out_color: state.rts().render2_w.clone(),
                };
                window.encoder.draw(&slice, &pso, &data);

                let pso = &state.psos().blur;
                let (ref vbuf, ref slice) = state.buffers().screen;
                let data = blur_pipeline::Data {
                    vbuf: vbuf.clone(),
                    u_model_view_proj: proj,
                    horizontal: 1,
                    in_color: (state.rts().render2_r.clone(), scale_clamp_sampler.clone()),
                    out_color: window.output_color.clone(),
                };
                window.encoder.draw(&slice, &pso, &data);
            });
        }

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
        }

        if let Some(input) = e.press_args() {
            use piston_window::Button::*;
            println!("{:?}", input);
            match input {
                Keyboard(Key::LCtrl) => { enabl = !enabl }
                _ => (),
            }
        }

    }
}
