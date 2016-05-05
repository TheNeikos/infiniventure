
use std::collections::HashMap;

use na::Vector3;
use ncollide::shape::Cuboid;

// The order is never to be changed, for now
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum FaceType {
    Stone = 0, Dirt, GrassDirt, Grass
}

#[derive(Clone, Debug)]
pub struct CubeFaces {
    top:    FaceType,
    bottom:  FaceType,
    right: FaceType,
    left:  FaceType,
    front: FaceType,
    back:  FaceType,
}

impl CubeFaces {
    pub fn to_array(&self) -> [i32; 3] {
        let mut ret = [0i32; 3];
        ret[0] = ((self.top as i32  ) << 16) + self.bottom as i32;
        ret[1] = ((self.right as i32) << 16) + self.left as i32;
        ret[2] = ((self.front as i32) << 16) + self.back as i32;
        return ret;
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum CubeType {
    Stone, Dirt, Grass
}

impl CubeType {
    fn to_cubefaces(&self) -> CubeFaces {
        use geo::CubeType::*;
        match *self {
            Stone => {
                CubeFaces {
                    top:    FaceType::Stone,
                    bottom:  FaceType::Stone,
                    right: FaceType::Stone,
                    left:  FaceType::Stone,
                    front: FaceType::Stone,
                    back:  FaceType::Stone,
                }
            },
            Dirt => {
                CubeFaces {
                    top:    FaceType::Dirt,
                    bottom:  FaceType::Dirt,
                    right: FaceType::Dirt,
                    left:  FaceType::Dirt,
                    front: FaceType::Dirt,
                    back:  FaceType::Dirt,
                }
            },
            Grass => {
                CubeFaces {
                    top:    FaceType::Grass,
                    bottom:  FaceType::Dirt,
                    right: FaceType::GrassDirt,
                    left:  FaceType::GrassDirt,
                    front: FaceType::GrassDirt,
                    back:  FaceType::GrassDirt,
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub faces: CubeFaces,
    pub shape: Cuboid<Vector3<f32>>,
    pub kind:  CubeType,
    pub pos: Vector3<f32>,
}

impl Cube {
    pub fn new(pos: Vector3<f32>, size: Vector3<f32>, kind: CubeType) -> Cube {
        Cube {
            pos: pos,
            shape: Cuboid::new(size/4.0),
            faces: kind.to_cubefaces(),
            kind: kind,
        }
    }

    pub fn pos(&self) -> [f32; 3] {
        [self.pos.x, self.pos.y, self.pos.z]
    }
}

pub type ChunkSize = u8;
pub type BlockPos = (ChunkSize, ChunkSize, ChunkSize);

pub struct Chunk {
    blocks: HashMap<BlockPos, Cube>,
}

