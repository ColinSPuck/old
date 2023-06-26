// main.rs
#![feature(portable_simd)]

/*
pub mod octree;
pub mod builder;
pub mod types;
pub mod lab2;
pub mod util;
*/

pub fn main() {
    //Create instance
    use vulkano::VulkanLibrary;
    use vulkano::instance::{Instance, InstanceCreateInfo};
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default())
    .expect("failed to create instance");

    //enumerate physical devices
    let physical_device = instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");

    //queue family device enumeration
    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s)", family.queue_count);
    }

    //Creating a device:

    //locate a queue family that supports graphical operations
    use vulkano::device::QueueFlags;

    let queue_family_index = physical_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_queue_family_index, queue_family_properties)| {
        queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
    })
    .expect("couldn't find a graphical queue family") as u32;

    //create the device with the index of the viable queue family
    use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    //Creating a device returns the device and a list of queue objects.



    //Creating a memory allocator

    use vulkano::memory::allocator::StandardMemoryAllocator;

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
    use vulkano::memory::allocator::{AllocationCreateInfo, MemoryUsage};

    let data: i32 = 12;
    let buffer = Buffer::from_data(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        data,
    )
    .expect("failed to create buffer");
}

