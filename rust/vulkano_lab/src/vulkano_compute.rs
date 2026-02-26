/// CPU 执行 Rust 程序，而 GPU 则是试图与之交互的对象。
/// 中央处理器（CPU）和图形处理器（GPU）都是逐条执行指令。在 CPU 上运行的常规程序可用的指令包括，例如，修改内存中的某个值，或者执行某些数学运算。
/// GPU 能够执行的指令通常有限，但它们能够同时处理大量数据。例如，您可以指示 GPU 将 32 个值乘以一个常数，这大约与 CPU 将单个值乘以该常数所需的时间相同（忽略在两个设备之间传输数据的开销）。
pub mod cs {
    use crate::vulkano_compute::cs;
    use crate::vulkano_physical_devices::create_device_queue;
    use std::sync::Arc;
    use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
    use vulkano::command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    };
    use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
    use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
    use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
    use vulkano::memory::allocator::{
        AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator,
    };
    use vulkano::pipeline::compute::ComputePipelineCreateInfo;
    use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
    use vulkano::pipeline::{
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo,
    };
    use vulkano::sync;
    use vulkano::sync::GpuFuture;

    // 使用GLSL编写一段程序，供GPU执行，在GPU上执行的程序为着色器（Shader）。
    // 通过宏将 GLSL代码嵌入到rust代码中
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } buf;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                buf.data[idx] *= 12;
            }
        ",
    }
    pub fn compute_operation() -> String {
        if let (Some(device), Some(queue)) = create_device_queue() {
            let memory_locator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
            let data_iter = 0..65536u32;
            let data_buffer = Buffer::from_iter(
                memory_locator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                data_iter,
            )
            .expect("failed to create buffer");

            // 1,将着色器传递给vulkan实现，创建计算管线
            let shader = cs::load(device.clone()).expect("Failed to create shader module");
            let cs = shader
                .entry_point("main")
                .expect("Failed to find entry point");
            let stage = PipelineShaderStageCreateInfo::new(cs);
            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            )
            .unwrap();
            let compute_pipeline = ComputePipeline::new(
                device.clone(),
                None,
                ComputePipelineCreateInfo::stage_layout(stage, layout),
            )
            .expect("Failed to create compute pipeline");

            // 2.创建描述符集分配器
            let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
                device.clone(),
                Default::default(),
            ));
            let pipeline = compute_pipeline.layout();
            let descriptor_set_layouts = pipeline.set_layouts();
            let descriptor_set_layout_index = 0;
            // 获取第一个描述符集布局
            let descriptor_set_layout = descriptor_set_layouts
                .get(descriptor_set_layout_index)
                .unwrap();
            let descriptor_set = DescriptorSet::new(
                descriptor_set_allocator.clone(),
                descriptor_set_layout.clone(),
                [WriteDescriptorSet::buffer(0, data_buffer.clone())],
                [],
            )
            .unwrap();

            //3.创建命令缓冲分配器
            let command_buffer_locator = Arc::new(StandardCommandBufferAllocator::new(
                device.clone(),
                StandardCommandBufferAllocatorCreateInfo::default(),
            ));

            let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                command_buffer_locator.clone(),
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            unsafe {
                command_buffer_builder
                    .bind_pipeline_compute(compute_pipeline.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        PipelineBindPoint::Compute,
                        compute_pipeline.layout().clone(),
                        descriptor_set_layout_index as u32,
                        descriptor_set,
                    )
                    .unwrap()
                    .dispatch([1024, 1, 1])
                    .unwrap();
            }
            let command_buffer = command_buffer_builder.build().unwrap();

            // 4.提交命令缓冲区
            let future = sync::now(device.clone())
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_signal_fence_and_flush()
                .unwrap();
            future.wait(None).unwrap();

            //5.读取结果
            let content = data_buffer.read().unwrap();
            return format!("{:?}", &content.iter().as_slice()[0..64]);
        }
        String::from("Nothing")
    }
}
