

// Create a new voxel octree.
fn new_voxel_octree() -> VoxelOctree {
    // Initialize the octree with a root node.
    let root_node = VoxelNode::new();
  
    // Return the octree.
    VoxelOctree { root_node }
  }
  
  // Add a voxel to the octree.
  fn add_voxel(octree: &mut VoxelOctree, x: i32, y: i32, z: i32) {
    // Get the current node.
    let mut current_node = &mut octree.root_node;
  
    // Recursively descend the octree until we reach a leaf node.
    while current_node.is_leaf() == false {
      // Get the child node that contains the voxel.
      let child_node = current_node.get_child(x, y, z);
  
      // If the child node does not exist, create it.
      if child_node.is_none() {
        child_node = Some(VoxelNode::new());
        current_node.set_child(x, y, z, child_node.unwrap());
      }
  
      // Move on to the child node.
      current_node = child_node.unwrap();
    }
  
    // Set the voxel value in the leaf node.
    current_node.set_voxel(x, y, z, true);
  }
  
  // Remove a voxel from the octree.
  fn remove_voxel(octree: &mut VoxelOctree, x: i32, y: i32, z: i32) {
    // Get the current node.
    let mut current_node = &mut octree.root_node;
  
    // Recursively descend the octree until we reach a leaf node.
    while current_node.is_leaf() == false {
      // Get the child node that contains the voxel.
      let child_node = current_node.get_child(x, y, z);
  
      // If the child node does not exist, return.
      if child_node.is_none() {
        return;
      }
  
      // Move on to the child node.
      current_node = child_node.unwrap();
    }
  
    // Set the voxel value in the leaf node to false.
    current_node.set_voxel(x, y, z, false);
  
    // If the leaf node is empty, remove it.
    if current_node.is_empty() {
      octree.root_node.remove_child(x, y, z);
    }
  }
  
  // Get the voxel value at a given point.
  fn get_voxel(octree: &VoxelOctree, x: i32, y: i32, z: i32) -> bool {
    // Get the current node.
    let current_node = &octree.root_node;
  
    // Recursively descend the octree until we reach a leaf node.
    while current_node.is_leaf() == false {
      // Get the child node that contains the voxel.
      let child_node = current_node.get_child(x, y, z);
  
      // If the child node does not exist, return false.
      if child_node.is_none() {
        return false;
      }
  
      // Move on to the child node.
      current_node = child_node.unwrap();
    }
  
    // Return the voxel value in the leaf node.
    current_node.get_voxel(x, y, z)
  } 