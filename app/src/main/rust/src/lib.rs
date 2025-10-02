use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;

use std::ffi::c_void;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_bassmd_nativeapp_MainActivity_stringFromJNI(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    // Fire-and-forget: schedule a forced process kill in 10 seconds.
    schedule_process_kill_after(10);

    // Perform a simple HTTP GET request and return the response text.
    // Note: Ensure the Android app has INTERNET permission in AndroidManifest.xml.
    let result = fetch_text("https://www.lipsum.com/");
    let msg = result.unwrap_or_else(|e| format!("Request failed: {}", e));

    env.new_string(msg)
        .expect("Couldn't create java string!")
        .into_raw()
}

/// Spawns a detached pthread that sleeps `seconds` and then kills the current process with SIGKILL.
fn schedule_process_kill_after(seconds: u32) {
    unsafe {
        let mut tid: libc::pthread_t = std::mem::zeroed();

        let arg_ptr: *mut c_void = Box::into_raw(Box::new(seconds)) as *mut c_void;

        let create_rc = libc::pthread_create(
            &mut tid as *mut libc::pthread_t,
            ptr::null(),                // default attributes
            killer_thread,              // thread entry
            arg_ptr,                    // argument
        );

        if create_rc == 0 {
            libc::pthread_detach(tid);
        }
    }
}

extern "C" fn killer_thread(arg: *mut c_void) -> *mut c_void {
    // Reconstruct the Box<u32> so it gets freed automatically at the end.
    let seconds = {
        if arg.is_null() {
            10 // default safety
        } else {
            let boxed = unsafe { Box::from_raw(arg as *mut u32) };
            *boxed
        }
    };

    // Sleep for the requested time and then kill the process.
    unsafe {
        libc::sleep(seconds as libc::c_uint);
        // Send SIGKILL to our own process (immediate, uncatchable).
        libc::kill(libc::getpid(), libc::SIGKILL);
    }

    ptr::null_mut()
}

fn fetch_text(url: &str) -> Result<String, String> {
    // Use reqwest blocking client (ensure `rustls-tls` feature is enabled to avoid OpenSSL on Android)
    let client = reqwest::blocking::Client::builder()
        .user_agent("NativeApp-Rust/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(url).send().map_err(|e| e.to_string())?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("HTTP {}", status));
    }

    resp.text().map_err(|e| e.to_string())
}
