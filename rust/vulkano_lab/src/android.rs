use crate::vulkano_buffer::create_buffer;
use crate::vulkano_compute::cs::compute_operation;
use crate::vulkano_image::use_vulkano_create_image;
use crate::vulkano_physical_devices::{
    collect_devices_queues_info, collect_physical_devices_infos, create_device_queue,
};
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;
use serde_json::json;
use std::ffi::CString;
use std::panic::catch_unwind;

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
    let rust_string = match create_device_queue() {
        Ok(_) => String::from("create queue and device successful!"),
        Err(err) => format!("create queue and device failed: {err}"),
    };
    let c_string = CString::new(rust_string).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_createVulkanoBuffer(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let create_res = create_buffer();
    let c_string = CString::new(create_res).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}
#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_vulkanoCompute(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let create_res = compute_operation();
    let c_string = CString::new(create_res).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}

#[unsafe(no_mangle)]
extern "system" fn Java_com_example_vulkanoapp_jni_VulkanoLab_vulkanoCreateImage(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let create_res = catch_unwind(|| {
        use_vulkano_create_image();
        "create image successful!"
    })
    .unwrap_or("create image failed!");
    let c_string = CString::new(create_res).expect("CString::new failed");
    let java_string = env
        .new_string(c_string.to_str().unwrap())
        .expect("Failed to create Java string");
    java_string.into_raw()
}
