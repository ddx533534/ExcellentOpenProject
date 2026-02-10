use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_helloVulkano(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let rust_string = "Hello from Rust!";
    let c_string = CString::new(rust_string).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhysicalDeviceInfo {
    name: String,
    device_type: String,
    api_version: String,
    driver_version: String,
}

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_vulkanoInfo(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let library = VulkanLibrary::new().expect("Unable to load vulkan library");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("Unable to create instance");
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Unable to get physical devices")
        .next()
        .expect("no physical device");

    let physical_device_info = PhysicalDeviceInfo {
        name: format!("{:?}", physical_device.properties().device_name),
        device_type: format!("{:?}", physical_device.properties().device_type),
        api_version: format!("{:?}", physical_device.properties().api_version),
        driver_version: format!("{:?}", physical_device.properties().driver_version),
    };
    let rust_string =
        serde_json::to_string(&physical_device_info).expect("Failed to serialize to JSON");

    let c_string = CString::new(rust_string).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}
