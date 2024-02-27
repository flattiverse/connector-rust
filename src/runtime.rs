use std::future::Future;
use std::time::{Duration, SystemTime};

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
    std::time::UNIX_EPOCH + Duration::from_secs_f64(time / 1000.0)
}

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
pub fn now() -> SystemTime {
    SystemTime::now()
}
