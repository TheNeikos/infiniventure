
use std::collections::HashMap;
use std::path::PathBuf;

use piston_window::*;
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;

use render::{VertexBuffer, ScreenVertex, Vertex, PSOBuffer, RTBuffer};

type Vert<T> = (gfx::handle::Buffer<gfx_device_gl::Resources, T>, gfx::Slice<gfx_device_gl::Resources>);

pub struct State<R: gfx::Resources = gfx_device_gl::Resources> {
    textures: HashMap<&'static str, Texture<R>>,
    buffers: BBuffer,
    psos: PSOBuffer,
    rts: RTBuffer,
}

impl<R: gfx::Resources> State<R> {
    pub fn buffers(&self) -> &BBuffer {
        &self.buffers
    }

    pub fn get_texture(&self, key: &'static str) -> Option<&Texture<R>> {
        self.textures.get(key)
    }

    pub fn psos(&self) -> &PSOBuffer {
        &self.psos
    }

    pub fn rts(&self) -> &RTBuffer {
        &self.rts
    }
}

impl State {
    pub fn new<W: Window>(window: &mut PistonWindow<W>) -> State {
        let mut state = State {
            textures: HashMap::new(),
            buffers: BBuffer {
                cube: initialize_cube(window),
                screen: initialize_screen(window),
            },
            psos: PSOBuffer::new(window),
            rts: RTBuffer::new(window),
        };
        load_texture(&mut state, window, "sprite.png");

        return state;
    }
}

fn load_texture<W: Window>(state: &mut State, w: &mut PistonWindow<W>, name: &'static str) {
    let mut path = PathBuf::from("assets");
    path.push(name);
    let tex = Texture::from_path(
        &mut w.factory,
        &path,
        Flip::None,
        &TextureSettings::new(),
        ).unwrap();
    state.textures.insert(name, tex);
}


pub struct BBuffer<R: gfx::Resources = gfx_device_gl::Resources> {
    pub cube: (gfx::handle::Buffer<R, Vertex>, gfx::Slice<R>),
    pub screen: (gfx::handle::Buffer<R, ScreenVertex>, gfx::Slice<R>)
}

fn initialize_cube<W: Window>(window: &mut PistonWindow<W>) -> Vert<Vertex>
{
    let vertex_data = vec![
        //top (0, 0, 0.5)
        Vertex::new([-0.5, 0.5,  -0.5], [0, 0], 0), //0
        Vertex::new([0.5,  0.5,  -0.5], [1, 0], 0), //1
        Vertex::new([0.5,  0.5,  0.5],  [1, 1], 0), //2
        Vertex::new([-0.5, 0.5,  0.5],  [0, 1], 0), //3
        //bottom(0,0,-0.5)
        Vertex::new([0.5,  -0.5, 0.5],  [0, 0], 1), //4
        Vertex::new([-0.5, -0.5, 0.5],  [1, 0], 1), //5
        Vertex::new([-0.5, -0.5, -0.5], [1, 1], 1), //6
        Vertex::new([0.5,  -0.5, -0.5], [0, 1], 1), //7
        //right(0.5,0,0)
        Vertex::new([0.5,  0.5,  0.5],  [0, 0], 2), //1
        Vertex::new([0.5,  0.5,  -0.5], [1, 0], 2), //11
        Vertex::new([0.5,  -0.5, -0.5], [1, 1], 2), //8
        Vertex::new([0.5,  -0.5, 0.5],  [0, 1], 2), //9
        //left(-0.5,0,0)
        Vertex::new([-0.5, 0.5,  0.5],  [0, 0], 3), //1
        Vertex::new([-0.5, 0.5,  -0.5], [1, 0], 3), //13
        Vertex::new([-0.5, -0.5, -0.5], [1, 1], 3), //14
        Vertex::new([-0.5, -0.5, 0.5],  [0, 1], 3), //15
        //front(0,0.5,0)
        Vertex::new([0.5,  0.5,  0.5],  [0, 0], 4), //1
        Vertex::new([-0.5, 0.5,  0.5],  [1, 0], 4), //19
        Vertex::new([-0.5, -0.5, 0.5],  [1, 1], 4), //16
        Vertex::new([0.5,  -0.5, 0.5],  [0, 1], 4), //17
        //back(0,-0.5,0)
        Vertex::new([0.5,  0.5,  -0.5], [0, 0], 5), //2
        Vertex::new([-0.5, 0.5,  -0.5], [1, 0], 5), //21
        Vertex::new([-0.5, -0.5, -0.5], [1, 1], 5), //22
        Vertex::new([0.5,  -0.5, -0.5], [0, 1], 5), //23
        ];

    let index_data: &[u8] = &[
        0,  2,  1,  0,  3,  2,  // top
        4,  5,  6,  4,  6,  7,  // bottom
        8,  10, 9,  8, 11,  10, // right
        12, 13, 14, 12, 14, 15, // left
        16, 17, 18, 16, 18, 19, // front
        20, 22, 21, 20, 23, 22, // back
    ];

    let data = window.factory.create_vertex_buffer_indexed(&vertex_data, index_data);

    data
}

fn initialize_screen<W: Window>(window: &mut PistonWindow<W>) -> Vert<ScreenVertex>
{
    let vertex_data = vec![
        ScreenVertex::new([-1.0, -1.0]),
        ScreenVertex::new([ 1.0, -1.0]),
        ScreenVertex::new([ 1.0,  1.0]),
        ScreenVertex::new([-1.0,  1.0]),
    ];

    let index_data: &[u8] = &[
        0, 2, 1, 0, 3, 2,
    ];

    let data = window.factory.create_vertex_buffer_indexed(&vertex_data, index_data);

    data
}
