use crate::vulkano_physical_devices::{
    collect_devices_queues_info, collect_physical_devices_infos, create_device_queue,
};
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;
use libloading::Library;
use serde_json::json;
use std::env;
use std::ffi::CString;
use std::sync::Arc;
use vulkano::device::{Device, Queue};

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

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_vulkanoInfo(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let physical_device_info = collect_physical_devices_infos();
    let device_queue_info = collect_devices_queues_info();
    let phy_str = serde_json::to_string(&physical_device_info)
        .expect("physical_device_info serde json failed");
    let queue_str =
        serde_json::to_string(&device_queue_info).expect("device_queue_info serde json failed");

    // 使用 json! 宏来创建包含这两个字段的 JSON 对象
    let final_json = json!({
        "physical_device_info": serde_json::Value::String(phy_str),
        "device_queue_info": serde_json::Value::String(queue_str),
    })
    .to_string();

    let c_string = CString::new(final_json).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_createVulkanoDeviceQueue(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    unsafe {
        let create_res = create_device_queue();
        let rust_string = match create_res {
            (_, _) => "create queue and device successful!",
        };
        let c_string = CString::new(rust_string).expect("CString::new failed");
        let java_string = env
            .new_string(c_string.to_str().unwrap())
            .expect("Failed to create Java string");
        java_string.into_raw()
    }
}
