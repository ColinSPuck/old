// types.rs
use glam::*;
use glium::*;
use bitflags::*;

#[derive(Clone)]
pub struct Voxel {
    pub material: u16, 
    pub leaf: bool
}

pub struct OctreeNode {
    pub bounds: Bounds, 
    pub children: [Option<Box<OctreeNode>>; 8],
    pub voxels: Vec<Voxel>
}

pub struct VoxelOctree {
    tree_depth: u32,
    buffer: UnmanagedByteBuffer,
}
// A voxel octant refers to an octant of an octree that is used to represent a voxel grid.
bitflags! {
    pub struct VoxelOctant: u8 {
        const Z0Y0X0 = 0b000;
        const Z0Y1X0 = 0b001;
        const Z1Y0X0 = 0b010;
        const Z0Y0X1 = 0b100;
        const Z0Y1X1 = 0b101;
        const Z1Y0X1 = 0b110;
        const Z1Y1X0 = 0b011;
        const Z1Y1X1 = 0b111;
    }
}
/*
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,  
    pub t: f32,
    pub object_id: Option<ObjectId>,  
    pub ray_id: u32
}
*/
#[derive(Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub size: [f32; 3],
    pub ambient: [f32; 4],
    pub normal: [f32; 3],
}
implement_vertex!(Vertex, position, size, ambient, normal);

pub struct UnmanagedByteBuffer {
    pub tree_data: *mut u16,
    pub capacity: usize
}

pub struct VoxelOctreeBuilder {
    default_material: u16,
    tree_depth: u32,
    voxel_data: multiarray::Array3D<u16>,
}