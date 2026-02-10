use image::{ImageBuffer, Rgba};
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyImageToBufferInfo,
};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::format::{ClearColorValue, Format};
use vulkano::image::{Image, ImageCreateInfo, ImageType};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::library::VulkanLibrary;
use vulkano::memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator};
use vulkano::sync::{GpuFuture, now};

pub fn use_vulkano() {
    // 1.加载vulkan系统库
    let library = VulkanLibrary::new().expect("Unable to load vulkan library");
    // 2.创建vulkan实例
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("Unable to create instance");
    // 3.寻找可用物理设备
    // 运行程序的机器可能有多个支持 Vulkan 的设备。在我们要求显卡执行某些操作之前，必须枚举所有支持 Vulkan 的物理设备，并选择要用于此操作的设备。
    // 实际上，物理设备可以是独立显卡，也可以是集成显卡处理器，甚至还可以是软件实现。它基本上可以是任何能够运行 Vulkan 操作的东西。
    // 比如在一台华为mate30上，可用设备为独立显卡，
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Unable to get physical devices")
        .next()
        .expect("no physical device");

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_size, properties)| properties.queue_flags.contains(QueueFlags::GRAPHICS))
        .expect("Unable to find queue flags") as u32;

    //4.创建物理设备对应的逻辑设备
    let (device, mut queue) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Unable to create logic device");

    // 5.获取可用任务队列
    let queue = queue.next().expect("Unable to get next logic queue");

    // 6.图像创建
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            array_layers: 1,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    )
    .unwrap();

    // 7.创建命令缓冲区分配器
    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let buf = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("Unable to create buffer");

    // 8.图像清除
    let mut builder = AutoCommandBufferBuilder::primary(
        Arc::new(command_buffer_allocator),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0., 0., 1., 1.]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buf.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    // 9.输出到图片
    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, buffer_content).unwrap();
    image.save("image.png").unwrap();
}

