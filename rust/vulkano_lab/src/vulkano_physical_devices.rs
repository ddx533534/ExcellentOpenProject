use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use vulkano::VulkanLibrary;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
/// 寻找可用物理设备。
/// 运行程序的机器可能有多个支持 Vulkan 的设备。在要求显卡执行某些操作之前，必须枚举所有支持 Vulkan 的物理设备，并选择要用于此操作的设备。
/// 实际上，物理设备可以是独立显卡，也可以是集成显卡处理器，甚至还可以是软件实现。它基本上可以是任何能够运行 Vulkan 操作的东西。
/// 比如在一台华为mate30上，可用设备为集成显卡，设备类型为 Mali-G76，api_version：1.1.191。
/// 在mac os上一台Android模拟器，可用设备为Cpu，设备类型为 SwiftShader Device(LLVM 10.0.0)，api_version：1.2.0。

#[derive(Clone)]
pub struct VulkanContext {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

static VULKAN_CONTEXT: OnceLock<VulkanContext> = OnceLock::new();

pub fn create_device_queue() -> Result<(Arc<Device>, Arc<Queue>), String> {
    if let Some(context) = VULKAN_CONTEXT.get() {
        return Ok((context.device.clone(), context.queue.clone()));
    }

    // 在部分华为机型上，Queue::drop 期间会触发驱动/layer 清理崩溃。
    // 这里缓存上下文，在应用生命周期内复用，避免在 JNI 方法返回时立即析构队列。
    let context = create_vulkan_context()?;
    let context = VULKAN_CONTEXT.get_or_init(|| context);

    Ok((context.device.clone(), context.queue.clone()))
}

fn create_vulkan_context() -> Result<VulkanContext, String> {
    // 1.加载vulkan系统库
    let library =
        VulkanLibrary::new().map_err(|err| format!("Unable to load vulkan library: {err}"))?;
    // 2.创建vulkan实例
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_layers: Vec::new(),
            ..Default::default()
        },
    )
    .map_err(|err| format!("Unable to create instance: {err}"))?;
    // 3.寻找第一个可用的支持vulkan的物理设备
    let available_physical_device = instance
        .enumerate_physical_devices()
        .map_err(|err| format!("Unable to get physical devices: {err}"))?
        .next()
        .ok_or_else(|| String::from("Unable to get physical devices"))?;
    // 4.获取可用物理设备上的可用队列族的索引
    let queue_family_index = available_physical_device
        .queue_family_properties()
        .iter()
        .position(|properties| properties.queue_flags.contains(QueueFlags::GRAPHICS))
        .ok_or_else(|| String::from("Unable to find a queue family"))?
        as u32;
    // 5.创建对应物理设备的逻辑设备和队列集合
    let (device, mut queues) = Device::new(
        available_physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .map_err(|err| format!("Unable to create logical device and queue: {err}"))?;

    // 6.从队列集合中取出第一个可用的队列
    let queue = queues
        .next()
        .ok_or_else(|| String::from("Unable to get available queue"))?;

    Ok(VulkanContext { device, queue })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalDeviceInfo {
    pub name: String,
    pub device_type: String,
    pub api_version: String,
    pub driver_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalDeviceQueueInfo {
    pub queue_count: u32,
    pub queue_flag: String,
}
pub fn collect_physical_devices() -> Vec<Arc<PhysicalDevice>> {
    // 1.加载vulkan系统库
    let library = VulkanLibrary::new().expect("Unable to load vulkan library");
    // 2.创建vulkan实例
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("Unable to create instance");
    let mut devices: Vec<Arc<PhysicalDevice>> = Vec::new();
    let _ = instance
        .enumerate_physical_devices()
        .expect("Unable to get physical devices")
        .for_each(|physical_device| {
            devices.push(physical_device.clone());
        });
    devices
}
pub fn collect_physical_devices_infos() -> Vec<PhysicalDeviceInfo> {
    let physical_devices = collect_physical_devices();
    let mut infos = Vec::new();
    let _ = physical_devices.iter().for_each(|physical_device| {
        infos.push(PhysicalDeviceInfo {
            name: format!("{:?}", physical_device.properties().device_name),
            device_type: format!("{:?}", physical_device.properties().device_type),
            api_version: format!("{:?}", physical_device.properties().api_version),
            driver_version: format!("{:?}", physical_device.properties().driver_version),
        })
    });
    infos
}

pub fn collect_devices_queues_info() -> Vec<PhysicalDeviceQueueInfo> {
    let physical_devices = collect_physical_devices();
    let mut infos: Vec<PhysicalDeviceQueueInfo> = Vec::new();
    let _ = physical_devices.iter().for_each(|physical_device| {
        physical_device
            .queue_family_properties()
            .iter()
            .for_each(|queue_family_properties| {
                infos.push(PhysicalDeviceQueueInfo {
                    queue_count: queue_family_properties.queue_count,
                    queue_flag: format!("{:?}", queue_family_properties.queue_flags),
                });
            });
    });
    infos
}
