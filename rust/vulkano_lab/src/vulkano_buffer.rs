use crate::vulkano_physical_devices::create_device_queue;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::sync;
use vulkano::sync::GpuFuture;

pub fn create_buffer() -> String {
    if let (Some(device), Some(queue)) = create_device_queue() {
        // 1.创建内存缓冲区分配器
        let memory_locator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let source_content = 0..64;

        // 2.源缓冲区
        let source = Buffer::from_iter(
            memory_locator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            source_content,
        )
        .expect("Failed to create src buffer");

        //3.目标缓冲区
        let destination_content = (0..64).map(|_| 1);
        let destination = Buffer::from_iter(
            memory_locator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            destination_content,
        )
        .expect("Failed to create dst buffer");

        //4.创建命令缓冲区分配器
        let command_buffer_locator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        //5.创建主命令缓冲区
        let mut builder = AutoCommandBufferBuilder::primary(
            command_buffer_locator.clone(),
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
            .unwrap();
        let primary_command_buffer = builder.build().unwrap();

        //6.创建执行对象
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), primary_command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let src = source.read().unwrap();
        let dst = destination.read().unwrap();
        return format!("{:?}\n{:?}", src.iter().as_slice(), dst.iter().as_slice());
    }
    String::from("Failed to copy buffer")
}
