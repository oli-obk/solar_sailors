use std::future::Future;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(target_arch = "wasm32"))]
pub fn set(key: &str, value: &str) {
    assert!(TRANSACTION.load(Ordering::Relaxed));
    let odd = ODD.load(Ordering::Relaxed);
    set_inner(&format!("{}/{}", odd as u8, key), value)
}

#[cfg(not(target_arch = "wasm32"))]
fn set_inner(key: &str, value: &str) {
    let path = path(key);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
        }
    }
    std::fs::write(path, value).unwrap();
}

#[cfg(target_arch = "wasm32")]
pub fn set(key: &str, value: &str) {
    quad_indexed_db::set(key, value)
}

#[cfg(not(target_arch = "wasm32"))]
fn path(key: &str) -> PathBuf {
    let mut path = directories::ProjectDirs::from("", "", "solar_sailors")
        .map(|dirs| dirs.data_local_dir().to_owned())
        .unwrap_or_default();

    for elem in key.split('/') {
        path.push(elem);
    }
    path
}

#[cfg(target_arch = "wasm32")]
pub fn get(key: &str) -> impl Future<Output = Option<String>> {
    quad_indexed_db::get(key)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get(key: &str) -> impl Future<Output = Option<String>> {
    // Only do it while in the "loading" stage, not during the game itself,
    // as you may get inconsistent state.
    assert!(!TRANSACTION.load(Ordering::Relaxed));
    // Always read from the last successful frame.
    // If there was no previous successful frame, immediately bail out, there can't
    // be any actual values anyway.
    let val = get_inner("odd").and_then(|odd| {
        let odd: bool = odd.parse().unwrap();
        get_inner(&format!("{}/{}", odd as u8, key))
    });
    async move { val }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_inner(key: &str) -> Option<String> {
    let path = path(key);
    std::fs::read_to_string(path).ok()
}

#[cfg(not(target_arch = "wasm32"))]
static ODD: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
static TRANSACTION: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "wasm32")]
pub async fn transaction_loop<F: Future<Output = ()>>(f: impl FnMut() -> F) {
    quad_indexed_db::transaction(f).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn transaction_loop<F: Future<Output = ()>>(mut f: impl FnMut() -> F) {
    assert_eq!(
        TRANSACTION.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed),
        Ok(false)
    );
    // Figure out the last successfull transaction.
    let mut odd = get_inner("odd").map(|s| s.parse().unwrap()).unwrap_or(true);
    loop {
        // Use the next frame.
        odd = !odd;
        // Preserve previous state.

        let dest = path(&(odd as u8).to_string());
        std::fs::remove_dir_all(&dest).unwrap();
        copy_dir::copy_dir(path(&((!odd) as u8).to_string()), dest).unwrap();
        // Let all the regular storage ops know what prefix to use.
        ODD.store(odd, Ordering::Relaxed);
        // Perform transaction
        f().await;
        // Transaction successfully done
        set_inner("odd", &odd.to_string());
    }
}
