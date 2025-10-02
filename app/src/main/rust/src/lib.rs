use jni::objects::JClass;
use jni::sys::{jstring};
use jni::JNIEnv;

// Java_com_<pkg with underscores>_<Class>_<method>
// com.bassmd.nativeapp -> com_bassmd_nativeapp
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_bassmd_nativeapp_MainActivity_stringFromJNI(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string("Hello from Rust")
        .expect("Couldn't create java string!")
        .into_raw()
}