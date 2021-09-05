#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn set(key: &str, value: &str) {
    assert!(TRANSACTION.load(Ordering::Relaxed));
    let odd = ODD.load(Ordering::Relaxed);
    set_inner(&format!("{}/{}", odd as u8, key), value)
}

fn set_inner(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        quad_storage_sys::set(key, value)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = path(key);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
        }
        std::fs::write(path, value).unwrap();
    }
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

pub fn get(key: &str) -> Option<String> {
    // Only do it while in the "loading" stage, not during the game itself,
    // as you may get inconsistent state.
    assert!(!TRANSACTION.load(Ordering::Relaxed));
    // Always read from the last successful frame.
    // If there was no previous successful frame, immediately bail out, there can't
    // be any actual values anyway.
    let odd: bool = get_inner("odd")?.parse().unwrap();
    get_inner(&format!("{}/{}", odd as u8, key))
}

fn get_inner(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        quad_storage_sys::get(key)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = path(key);
        std::fs::read_to_string(path).ok()
    }
}

static ODD: AtomicBool = AtomicBool::new(false);
static TRANSACTION: AtomicBool = AtomicBool::new(false);

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

        #[cfg(target_arch = "wasm32")]
        {
            use quad_storage_sys::*;
            let oddstr = format!("{}/", (!odd) as u8);
            for i in 0..len() {
                let key = key(i).unwrap();
                if key.starts_with(&oddstr) {
                    let new_key = format!("{}/{}", odd as u8, &key[1..]);
                    let val = get(&key).unwrap();
                    set(&new_key, &val);
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let dest = path(&(odd as u8).to_string());
            // Also work if there wasn't a save game yet so no folders, either
            if std::fs::remove_dir_all(&dest).is_ok() {
                copy_dir::copy_dir(path(&((!odd) as u8).to_string()), dest).unwrap();
            }
        }
        // Let all the regular storage ops know what prefix to use.
        ODD.store(odd, Ordering::Relaxed);
        // Perform transaction
        f().await;
        // Transaction successfully done
        set_inner("odd", &odd.to_string());
    }
}
