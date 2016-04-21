
use std::collections::HashMap;
use std::path::PathBuf;

use piston_window::*;
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;

use render::{VertexBuffer, Vertex, PSOBuffer};

pub struct State<R: gfx::Resources = gfx_device_gl::Resources> {
    vbuffers: HashMap<VertexBuffer, (gfx::handle::Buffer<R, Vertex>, gfx::Slice<R>)>,
    textures: HashMap<&'static str, Texture<R>>,
    psos: PSOBuffer,
}

impl<R: gfx::Resources> State<R> {
    pub fn get_buffer(&self, key: VertexBuffer)
        -> Option<&(gfx::handle::Buffer<R, Vertex>, gfx::Slice<R>)> {
        self.vbuffers.get(&key)
    }

    pub fn get_texture(&self, key: &'static str) -> Option<&Texture<R>> {
        self.textures.get(key)
    }

    pub fn get_psos(&self) -> &PSOBuffer {
        &self.psos
    }
}

impl State {
    pub fn new<U, W: Window>(window: &mut PistonWindow<U, W>) -> State {
        let mut state = State {
            vbuffers: HashMap::new(),
            textures: HashMap::new(),
            psos: PSOBuffer::new(window)
        };
        initialize_cube(&mut state, window);
        load_texture(&mut state, window, "sprite.png");

        return state;
    }
}

fn load_texture<R, W: Window>(state: &mut State, w: &mut PistonWindow<R, W>, name: &'static str) {
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


fn initialize_cube<R, W: Window>(state: &mut State, window: &mut PistonWindow<R, W>) {
    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([-1,    1, -1], [0, 0], 0), // 0
        Vertex::new([ 1,    1, -1], [1, 0], 0), // 1
        Vertex::new([ 1,    1,  1], [1, 1], 0), // 2
        Vertex::new([-1,    1,  1], [0, 1], 0), // 3
        //bottom (0, 0, -1)
        Vertex::new([ 1,   -1,  1], [0, 0], 1), // 4
        Vertex::new([-1,   -1,  1], [1, 0], 1), // 5
        Vertex::new([-1,   -1, -1], [1, 1], 1), // 6
        Vertex::new([ 1,   -1, -1], [0, 1], 1), // 7
        //right (1, 0, 0)
        Vertex::new([ 1,    1,  1], [0, 0], 2), // 10
        Vertex::new([ 1,    1, -1], [1, 0], 2), // 11
        Vertex::new([ 1,   -1, -1], [1, 1], 2), // 8
        Vertex::new([ 1,   -1,  1], [0, 1], 2), // 9
        //left (-1, 0, 0)
        Vertex::new([-1,    1,  1], [0, 0], 3), // 12
        Vertex::new([-1,    1, -1], [1, 0], 3), // 13
        Vertex::new([-1,   -1, -1], [1, 1], 3), // 14
        Vertex::new([-1,   -1,  1], [0, 1], 3), // 15
        //front (0, 1, 0)
        Vertex::new([ 1,    1,  1], [0, 0], 4), // 18
        Vertex::new([-1,    1,  1], [1, 0], 4), // 19
        Vertex::new([-1,   -1,  1], [1, 1], 4), // 16
        Vertex::new([ 1,   -1,  1], [0, 1], 4), // 17
        //back (0, -1, 0)
        Vertex::new([ 1,    1, -1], [0, 0], 5), // 20
        Vertex::new([-1,    1, -1], [1, 0], 5), // 21
        Vertex::new([-1,   -1, -1], [1, 1], 5), // 22
        Vertex::new([ 1,   -1, -1], [0, 1], 5), // 23
        ];

    let index_data: &[u8] = &[
        0,  1,  2,  2,  3,  0,  // top
        4,  6,  5,  6,  4,  7,  // bottom
        8,  9, 10, 10,  8,  11,  // right
        12, 14, 13, 14, 12, 15, // left
        16, 18, 17, 18, 16, 19, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let data = window.factory.create_vertex_buffer_indexed(&vertex_data, index_data);

    state.vbuffers.insert(VertexBuffer::Cube, data);
}
