//lab2.rs

use crate::*;
use crate::octree::*;

use bytemuck::*;
use lazy_static::*;

use std::*;


struct Array3D2P<T> {
    data: Vec<T>, 
    size: u32,
    size_power: u32
}

impl<T> Array3D2P<T> {
    pub fn new(element: T, size: u32) -> Self where T: Clone {
        let len = size * size * size;
        let mut data: Vec<T> = Vec::with_capacity(len as usize);
        let size_power: u32 = Self::exact_log(size);

        for i in 0..len {
            data.push(element.clone());
        }

        Array3D2P { data, size, size_power }
    }

    pub fn get(&self, pos: glam::UVec3) -> &T {
        self.data.get(Self::compute_index(self.size_power, pos)).unwrap()
    }
    
    pub fn get_mut(&mut self, pos: glam::UVec3) -> &mut T {
        self.data.get_mut(Self::compute_index(self.size_power, pos)).unwrap()
    }


    pub unsafe fn get_unchecked(&self, pos: glam::UVec3) -> &T {
        self.data.get_unchecked(Self::compute_index(self.size_power, pos))
    }

    pub unsafe fn get_unchecked_mut(&mut self, pos: glam::UVec3) -> &mut T { 
        self.data.get_unchecked_mut(Self::compute_index(self.size_power, pos))
    }

    fn compute_index(power: u32, pos: glam:: UVec3) -> usize {
        ((pos.z << (power << 1)) | (pos.y << power) | pos.x) as usize
    }

    fn exact_log(n: u32) -> u32 {
        let pow : u32 = n.trailing_zeros();
        assert! (n.leading_zeros() + pow == 31, "The given size was not a power of 2."); 
        pow
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
struct OctreeRenderAttribute {
    x: u8,
    y: u8,
    z: u8,
    flags: u8,
    size: u8,
    voxels: [u8; 64],
    material_pointer: u32,
    material_mask: u64,
    padding: [u8; 3],
}

impl OctreeRenderAttribute {
    pub fn new(x: u8, y: u8, z: u8, u: u8, v: u8, dir: Direction, voxels: &[u8; 64], material_pointer: u32, material_mask: u64) -> Self {
        let flags: u8 = dir.bits().trailing_zeros() as u8;
        let size: u8 = ((v + 1) << 3) | (u + 1);
        OctreeRenderAttribute {
            x,
            y,
            z,
            flags,
            size,
            voxels: *voxels,
            material_pointer,
            material_mask,
            padding: [0; 3],
        }
    }

    pub fn new_with_position(position: glam::UVec3, u: u8, v: u8, dir: Direction, voxels: &[u8; 64], material_pointer: u32, material_mask: u64) -> Self {
        Self::new(position.x as u8, position.y as u8, position.z as u8, u, v, dir, voxels, material_pointer, material_mask)
    }
}

#[derive(Copy, Clone, Zeroable)]
struct BoundingBox {
    x: u8,
    y: u8,
    z: u8,
    u: u8,
    v: u8,
    w: u8,
}

impl BoundingBox {
    pub fn new(start: glam::UVec3, ext: glam::UVec3) -> Self {
        let x = start.x as u8;
        let y = start.y as u8;
        let z = start.z as u8;
        let u = x + ext.x as u8;
        let v = y + ext.y as u8;
        let w = z + ext.z as u8;
        Self { x, y, z, u, v, w }
    }

    pub fn union(self, other: BoundingBox) -> Self {
        let x = std::cmp::min(self.x, other.x);
        let y = std::cmp::min(self.y, other.y);
        let z = std::cmp::min(self.z, other.z);
        let u = std::cmp::min(self.u, other.u);
        let v = std::cmp::min(self.v, other.v);
        let w = std::cmp::min(self.w, other.w);
        Self { x, y, z, u, v, w }
    }

    pub fn intersection(self, other: BoundingBox) -> Self {
        let x = std::cmp::max(self.x, other.x);
        let y = std::cmp::max(self.y, other.y);
        let z = std::cmp::max(self.z, other.z);
        let u = std::cmp::min(self.u, other.u);
        let v = std::cmp::min(self.v, other.v);
        let w = std::cmp::min(self.w, other.w);
        Self { x, y, z, u, v, w }
    }

    pub fn zero(&self) -> bool {
        self.u <= self.x || self.v <= self.y || self.w <= self.z || ((self.u - self.x) * (self.v - self.y) * (self.w - self.z)) == 0
    }
}

lazy_static! {
    static ref BOUNDING_BOX_MASK_TABLE: [BoundingBox; 256] = generate_bounding_box_mask_lookup_table();
}

fn generate_bounding_box_mask_lookup_table() -> [BoundingBox; 256] {
    let mut table = [BoundingBox::new(const_uvec3!([0, 0, 0]), const_uvec3!([0, 0, 0])); 256];

    for i in 0..256 {
        let mut aabb: Option<BoundingBox> = None;

        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    if (i & 1 << ((z << 2) | (y << 1) | x)) > 0 {
                        let b = BoundingBox::new(glam::uvec3(x, y, z), const_uvec3!([1, 1, 1]));
                        aabb = match aabb {
                            None => Some(b),
                            Some(x) => Some(x.union(b)),
                        };
                    }
                }
            }
        }

        table[i as usize] = aabb.unwrap_or(BoundingBox::new(const_uvec3!([0, 0, 0]), const_uvec3!([1, 1, 1])));
    }

    table
}

struct PositionedFace {
    x: u8,
    y: u8,
    z: u8,
    extents: FaceExtent,
}

impl PositionedFace {
    pub fn new(x: u8, y: u8, z: u8, extents: FaceExtent) -> Self {
        Self { x, y, z, extents }
    }
}

struct OctreeRenderBox {
    position: glam::UVec3,
    faces: [Option<FaceExtent>; 6],
    aabb: BoundingBox,
    voxels: [u8; 64],
}

impl OctreeRenderBox {
    const XY_MASK_TABLE: [[[[u64; 4]; 4]; 2]; 2] = Self::generate_xy_mask_lookup_table();

    pub fn create(position: glam::UVec3, faces: Direction, partials: Direction, map: &Array3D2P<OctreeBoxEntry>) -> Option<OctreeRenderBox> {
        unsafe {
            if faces == Direction::NONE {
                None
            } else {
                let mut aabb = None::<BoundingBox>;
                let mut voxels = [0u8; 64];
                let box_position = position >> const_uvec3!([3, 3, 3]);

                Self::fill_mask_array(map.get_unchecked(box_position).branch.as_ref().unwrap_unchecked(), position, &mut voxels, &mut aabb);
                let extents = Self::clip_face_extents(box_position, faces, partials, aabb, map);

                if let Some(ext) = extents {
                    Some(OctreeRenderBox {
                        position,
                        faces: ext,
                        aabb: aabb.unwrap_unchecked(),
                        voxels,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[inline(always)]
pub unsafe fn add_materials_to_texture(&self, texture: &mut UnmanagedByteBuffer<u16>, map: &Array3D2P<OctreeBoxEntry>) -> (u32, u64) {
    let pos = texture.count();
    texture.add((self.position.x ^ self.position.y ^ self.position.z) as u16);
    (pos, 0)
}

#[inline(always)]
pub fn add_faces_to_buffer(
    &self,
    map: &Array3D2P<OctreeBoxEntry>,
    back: &mut Vec<OctreeRenderAttribute>,
    material_pointer: u32,
    material_mask: u64,
) {
    if let Some(ext) = self.faces[0] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.x as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
    if let Some(ext) = self.faces[1] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.u as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
    if let Some(ext) = self.faces[2] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.x as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
    if let Some(ext) = self.faces[3] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.u as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
    if let Some(ext) = self.faces[4] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.x as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
    if let Some(ext) = self.faces[5] {
        back.push(OctreeRenderAttribute::new_with_position(
            self.position + glam::uvec3(self.aabb.x as u32, ext.x as u32, ext.y as u32),
            ext.width(),
            ext.height(),
            Direction,
            &self.voxels,
            material_pointer,
            material_mask,
        ));
    }
}

#[inline(always)]
unsafe fn clip_face_extents(
    box_position: glam::UVec3,
    faces: Direction,
    partials: Direction,
    aabb: Option<BoundingBox>,
    map: &Array3D2P<OctreeBoxEntry>,
) -> Option<[Option<FaceExtent>; 6]> {
    let aabb = aabb?;

    let mut extents: [Option<FaceExtent>; 6] = [None; 6];

    let aabb_face = FaceExtent::new_with_points(aabb.y, aabb.z, aabb.v, aabb.w);
    if aabb.x == 0 {
        if partials.contains(Direction::LEFT) {
            let pos = box_position - const_uvec3!([1, 0, 0]);
            extents[0] = Self::get_incoming_partial_extent(Direction::Right, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::LEFT) {
            extents[0] = Some(aabb_face);
        }
    } else {
        extents[0] = Some(aabb_face);
    }
    if aabb.u == 8 {
        if partials.contains(Direction::LEFT) {
            let pos = box_position - const_uvec3!([1, 0, 0]);
            extents[1] = Self::get_incoming_partial_extent(Direction::LEFT, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::LEFT) {
            extents[1] = Some(aabb_face);
        }
    } else {
        extents[1] = Some(aabb_face);
    }

    let aabb_face = FaceExtent::new_with_points(aabb.x, aabb.z, aabb.u, aabb.w);
    if aabb.y == 0 {
        if partials.contains(Direction::DOWN) {
            let pos = box_position - const_uvec3!([0, 1, 0]);
            extents[2] = Self::get_incoming_partial_extent(Direction::Right, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::DOWN) {
            extents[2] = Some(aabb_face);
        }
    } else {
        extents[2] = Some(aabb_face);
    }
    if aabb.u == 8 {
        if partials.contains(Direction::UP) {
            let pos = box_position - const_uvec3!([0, 1, 0]);
            extents[3] = Self::get_incoming_partial_extent(Direction::LEFT, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::UP) {
            extents[3] = Some(aabb_face);
        }
    } else {
        extents[3] = Some(aabb_face);
    }

    let aabb_face = FaceExtent::new_with_points(aabb.x, aabb.y, aabb.u, aabb.v);
    if aabb.z == 0 {
        if partials.contains(Direction::BACK) {
            let pos = box_position - const_uvec3!([0, 0, 1]);
            extents[4] = Self::get_incoming_partial_extent(Direction::FRONT, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::BACK) {
            extents[4] = Some(aabb_face);
        }
    } else {
        extents[4] = Some(aabb_face);
    }
    if aabb.w == 8 {
        if partials.contains(Direction::FRONT) {
            let pos = box_position + const_uvec3!([0, 0, 1]);
            extents[5] = Self::get_incoming_partial_extent(Direction::BACK, aabb_face, map.get_unchecked(pos));
        } else if faces.contains(Direction::FRONT) {
            extents[5] = Some(aabb_face);
        }
    } else {
        extents[5] = Some(aabb_face);
    }

    if extents.iter().any(|x| x.is_some()) {
        Some(extents)
    } else {
        None
    }
}

#[inline(always)]
fn get_incoming_partial_extent(direction: Direction, extent: FaceExtent, entry: &OctreeBoxEntry) -> Option<FaceExtent> {
    unsafe {
        let mut result = None;
        scan_branch_for_transparency(entry.branch.as_ref().unwrap_unchecked(), direction, extent, &mut result);
        result
    }
}

#[inline(always)]
fn scan_branch_for_transparency(reader: &VoxelOctreeReader, face: Direction, constraint: FaceExtent, ext: &mut Option<FaceExtent>) {
    const FACE_TABLE: [[u8; 4]; 6] = [
        [VoxelOctant::Z0Y0X0.bits(), VoxelOctant::Z0Y1X0.bits(), VoxelOctant::Z1Y0X0.bits(), VoxelOctant::Z1Y1X0.bits()],
        [VoxelOctant::Z0Y0X1.bits(), VoxelOctant::Z0Y1X1.bits(), VoxelOctant::Z1Y0X1.bits(), VoxelOctant::Z1Y1X1.bits()],
        [VoxelOctant::Z0Y0X0.bits(), VoxelOctant::Z0Y0X1.bits(), VoxelOctant::Z1Y0X0.bits(), VoxelOctant::Z1Y0X1.bits()],
        [VoxelOctant::Z0Y1X0.bits(), VoxelOctant::Z0Y1X1.bits(), VoxelOctant::Z1Y1X0.bits(), VoxelOctant::Z1Y1X1.bits()],
        [VoxelOctant::Z0Y0X0.bits(), VoxelOctant::Z0Y0X1.bits(), VoxelOctant::Z0Y1X0.bits(), VoxelOctant::Z0Y1X1.bits()],
        [VoxelOctant::Z1Y0X0.bits(), VoxelOctant::Z1Y0X1.bits(), VoxelOctant::Z1Y1X0.bits(), VoxelOctant::Z1Y1X1.bits()],
    ];

    // TODO: Continue script
}