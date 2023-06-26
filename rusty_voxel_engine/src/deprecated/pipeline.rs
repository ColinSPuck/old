// init_device.rs (instance, device, memory allocator)
use vulkano::instance::Instance;
use vulkano::device::Device;
use vulkano::memory::allocator::StandardMemoryAllocator;


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