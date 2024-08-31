mod atomics;
pub use atomics::*;

use std::future::Future;
use std::time::SystemTime;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
pub fn spawn(f: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
pub fn spawn(f: impl Future<Output = ()> + Send + 'static) {
    tokio::runtime::Handle::current().spawn(f);
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
pub fn now() -> SystemTime {
    let time = web_sys::js_sys::Date::now();
    std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(time / 1000.0)
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
pub fn now() -> SystemTime {
    SystemTime::now()
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
pub fn format_date_time(time: SystemTime) -> String {
    let time = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let time = wasm_bindgen::JsValue::from_f64(time.as_secs_f64());
    web_sys::js_sys::Date::new(&time)
        .to_time_string()
        .as_string()
        .unwrap_or_default()
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
pub fn format_date_time(time: SystemTime) -> String {
    let time = chrono::DateTime::<chrono::Local>::from(time);
    let time = time.naive_local();
    time.format("%T.%3f").to_string()
}
