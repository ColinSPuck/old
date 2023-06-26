//builder.rs
//deprecated

use glam::Vec3;
use parking_lot::RwLock;
use std::sync::Arc;
use crate::octree::{Voxel, Node};

pub trait OctreeBuilder<Voxel> {
    fn new(size: i32, max_depth: i32) -> Self;
    fn insert(&mut self, point: Vec3, voxel: Voxel);
    fn get_octant(&self, pos: &Vec3) -> OctreeBuilderResult<Voxel>; 
    fn get_block(&self, pos: &Vec3) -> [u16; 8];  
}

pub enum OctreeBuilderResult<Voxel> {
    Homogeneous(u16), 
    Sparse(),
    SamplingRequired(Voxel)
}

pub struct SparseVoxelOctree {
    pub size: i32,
    pub max_depth: i32,
    pub root: Option<Arc<RwLock<Node>>>,
}

impl SparseVoxelOctree {
    pub fn new(size: i32, max_depth: i32) -> Self {
        Self {
            size,
            max_depth,
            root: None,
        }
    }

    pub fn insert(&mut self, point: Vec3, voxel: Voxel) {
        let point = Vec3::new(point.x.floor(), point.y.floor(), point.z.floor());
        Self::insert_impl(&mut self.root, self.size as f32, self.max_depth, point, voxel, point, 0);
    }

    fn insert_impl(
        node: &mut Option<Arc<RwLock<Node>>>,
        size: f32,
        max_depth: i32,
        point: Vec3,
        voxel: Voxel,
        position: Vec3,
        depth: i32,
    ) {
        let size = size / f32::powi(2.0, depth);
    
        if depth == max_depth || (point.x == position.x && point.y == position.y && point.z == position.z) {
            *node = Some(Arc::new(RwLock::new(Node::new_leaf(voxel))));
            return;
        }
    
        if node.is_none() {
            *node = Some(Arc::new(RwLock::new(Node::new_branch())));
        }
    
        let mut node = node.as_ref().unwrap().write();
        let child_index = (((point.x > 0.5) as usize) << 2)
            | (((point.y > 0.5) as usize) << 1)
            | ((point.z > 0.5) as usize);
        let new_point = (point - Vec3::new(0.5, 0.5, 0.5) * (child_index as f32)) * Vec3::new(2.0, 2.0, 2.0);
        Self::insert_impl(&mut node.children[child_index], size, max_depth, new_point, voxel, position, depth + 1);
    }
}
