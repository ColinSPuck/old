//octree.rs
//deprecated
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Voxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct Node {
    pub is_leaf: bool,
    pub children: Box<[Option<Arc<RwLock<Node>>>; 8]>,
    pub data: Voxel,
}

impl Node {
    pub fn new_leaf(voxel: Voxel) -> Self {
        Self {
            is_leaf: true,
            children: Default::default(),
            data: voxel,
        }
    }

    pub fn new_branch() -> Self {
        Self {
            is_leaf: false,
            children: Default::default(),
            data: Voxel { x: 0, y: 0, z: 0},
        }
    }
}
