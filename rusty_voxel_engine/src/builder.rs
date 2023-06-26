// builder.rs

use crate::types::*;

use std::simd::*;

pub trait OctreeBuilder<T> {
    fn default_state(&self) -> T;
    fn get_tree_depth(&self) -> u32;
    fn get_octant (&self, pos: &OctreeCreationPosition, buffer: &mut UnmanagedByteBuffer, state: &T) -> OctreeBuilderResult<T>; 
    fn get_block(&self, pos: &OctreeCreationPosition, buffer: &mut UnmanagedByteBuffer, state: &T) -> u16x8;
}

pub enum OctreeBuilderResult<T> {
    Homogeneous(u16),
    Sparse,
    SamplingRequired(T),
}
pub struct OctreeCreationPosition {
    position: glam::Vec3,
    level: u32,
    octant: VoxelOctant,
}

impl OctreeCreationPosition {
    pub fn new(position: glam::Vec3, level: u32, octant: VoxelOctant) -> Self {
        Self { position, level, octant }
    }

    pub fn position(&self) -> glam::Vec3 {
        self.position
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn octant (&self) -> VoxelOctant {
        self.octant
    }

    pub unsafe fn child_unchecked(&self, octant: VoxelOctant) -> OctreeCreationPosition {
        debug_assert!(self.level > 0);
        Self::new(self.position + (octant.into() << self.level), self.level - 1, octant)
    }
}

pub struct Array3DOctreeBuilder<'a> {
    array: &'a multiarray::Array3D<u16>,
    default_material: u16,
    tree_depth: u32,
}

impl<'a> Array3DOctreeBuilder<'a> {
    pub fn new(array: &'a multiarray::Array3D<u16>, default_material: u16) -> Self {
        let ext = array.extents();
        let size = std::cmp::max(ext[0], std::cmp::max(ext[1], ext[2])) as u32;
        Self { array, default_material, tree_depth: size }
    }

    fn get_block_continuous(&self, pos: glam::UVec3) -> [u16; 8] {
        let mut data: [u16; 8] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let (x, y, z) = (pos.x as usize, pos.y as usize, pos.z as usize);
        data[0] = self.array[[x, y, z]];
        data[1] = self.array[[x + 1, y, z]];
        data[2] = self.array[[x, y + 1, z]];
        data[3] = self.array[[x, y, z + 1]];
        data[4] = self.array[[x + 1, y, z + 1]];
        data[5] = self.array[[x, y + 1, z + 1]];
        data[6] = self.array[[x + 1, y + 1, z]];
        data[7] = self .array[[x + 1, y + 1, z + 1]];
    }
}