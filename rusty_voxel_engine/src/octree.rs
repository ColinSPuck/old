// Octree.rs
use crate::builder::*;
use crate::types::*;

use std::simd::*;
use bitflags::*;
use glam::*;
use std::iter::*;

impl UnmanagedByteBuffer {
    pub fn new_with_capacity(capacity: usize) -> Self {
        let data = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(capacity, std::mem::align_of::<u8>()).unwrap()) };
        Self { tree_data, capacity }
    }

    pub fn get_allocation(&self, position: usize) -> *mut u8 {
        assert!(position < self.capacity);
        unsafe { self.data.add(position) } 
    }

    pub fn drop_allocation(&mut self) {
        unsafe { std::alloc::dealloc(self.data, std::alloc::Layout::from_size_align(self.capacity, std::mem::align_of::<u8>()).unwrap()) };
    }
}

impl OctreeNode {
    fn traverse(&self, ray: &Ray, vertex_buffer: &mut VertexBuffer) -> Option<Intersection> {
        // Traversal and intersection code...
        
        // Check ray ID before intersection
        if ray.ray_id == object.ray_id {
            return None; // Object already intersected
        }
        
        // Perform intersection test...
        ray.object_id = Some(object.id); // Update closest if needed
        //ray.t = ...; todo implement
            
        // Update object's ray ID
        object.ray_id = ray.ray_id;  
        
        Some(intersection)
    }
}
impl Voxel {
    #[inline(always)]
    pub fn calculate_vertices(&self, mut callback: impl FnMut(&dyn Vertex)) {
        let offsets: [Vec3; 8] = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];

    let uvs: [Vec2; 6] = [
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, 0.0),
    ];
        
        let mut vertices = Vec::new();
        
        if self.normal == Vec3::new(0.0, 0.0, 1.0) {
            for i in 0..6 {
                let local_position = offsets[i] * self.size;
                let uv = uvs[i];
                vertices.push(Vertex { 
                    position: local_position, 
                    uvs,  
                    material_index: self.material_index
                });
            }
        }
        
        // Similar for back face
  
        vertices
    }   
}

/* // needs translated to rust
impl Vertex {
    #[inline(always)]
        let u32: i = gl_VertexIndex / 6; //gl_vertexindex is a placeholder for the index of the vertex array
        let u32: j = gl_VertexIndex % 6; 
        // Push created vertices
        let v = Vertex::new(deref(push.vertices[i]));
        
        if (v.normal.xyz = (0, 0, 1)) {
            i[6] = u32[](1, 0, 3, 1, 3, 2);
            local_position.xyz += offsets[i[j]] * v.size;
            ambient_uv.xy = vec2(uvs[j].x, 1 - uvs[j].y);
            //texture = v.textures[0];
        }
        
        if (v.normal.xyz = Vec3{x: 0, y: 0, z: -1}) {
            i[6] = u32[](4, 5, 6, 4, 6, 7),-
            local_position.xyz += offsets[i[j]] * v.size;
            ambient_uv.xy = uvs[j].yx;
            //texture = v.textures[1];
        }
        
        VertexBuffer::new(facade, &vertices)
    }
}
*/

pub struct VoxelOctreeBuilder {
    default_material: u16,
    tree_depth: u32,
    voxel_data: multiarray::Array3D<u16>,
}

impl VoxelOctreeBuilder {
    pub fn new(default_material: u16, tree_depth: u32, voxel_data: multiarray::Array3D<u16>) -> Self {
        Self { default_material, tree_depth, voxel_data }
    }
}

impl OctreeBuilder<u16> for VoxelOctreeBuilder {
    fn default_state(&self) -> u16 {
        self.default_material
    }

    fn get_tree_depth(&self) -> u32 {
        self.tree_depth
    }

    fn get_octant(&self, pos: &OctreeCreationPosition, buffer: &mut UnmanagedByteBuffer, state: &u16) -> OctreeBuilderResult<u16> {
        // Fetch the 8 voxels in this octant
        let voxels = self.get_voxels_in_octant(pos);
        
        // Check if all voxels are the same. If they are, return Homogeneous with the common value.
        if voxels.iter().all(|&v| v == voxels[0]) {
            return OctreeBuilderResult::Homogeneous(voxels[0]);
        }
        
        // If voxels are not all the same, further subdivide or declare it sparse.
        // This depends on the level in the tree and other factors specific to your use case.
    }

    fn get_block(&self, pos: &OctreeCreationPosition, buffer: &mut UnmanagedByteBuffer, state: &u16) -> u16x8 {
        // Fetch the 8 voxels in this block and return them as a SIMD vector.
        let voxels = self.get_voxels_in_block(pos);
        return u16x8::new(voxels[0], voxels[1], voxels[2], voxels[3], voxels[4], voxels[5], voxels[6], voxels[7]);
    }
    
}

impl VoxelOctree<T> {
    // Creates a new voxel octree
    pub fn new(tree_depth: u32, buffer: UnmanagedByteBuffer<u8>) -> Self {
        Self { tree_depth, buffer }
    }  

    // WIP Creates a new voxel octree from an octree builder
    pub fn from_builder<t>(builder: &impl OctreeBuilder<T>) -> Self {
        let tree_depth = builder.get_tree_depth();

        unsafe {
            let mut buffer = UnmanageedByteBuffer::<u8>::new_with_capacity(((2 << (3 * tree_depth)) >> 5) as usize);
            let res = VoxelOctree::create_octree_data_linear(OctreeCreationPosition::new(new::Vec3(0,0,0), tree_depth, VoxelOctant::Z0Y0X0), builder, &mut buffer, &builder.default_state());

            if let Some(mat) = res.matrial {
                self::fill_homogeneous_top(res.flags, mat, tree_depth, &mut buffer);
            }

            return VoxelOctree {tree_depth, buffer };
        }
    }

    // Verifies the octree
    pub fn verify(&self) -> Result<(), VoxelOctreeVerificationError> {
        unsafe {
            if self.verify_level(0, self. tree_depth)? == self.buffer().count() {
                Ok(())
            }
            else {
                Err(VoxelOctreeVerificationError::NonPackedOctree(self.buffer().count()))
            }
        }
    }
    pub fn traverse_renderable(&self) {
        // Traverse nodes that are within the camera frustum
    }

    pub fn render(&self) {
        // Render the octree using the GPU
    }   
}