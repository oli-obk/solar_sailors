use std::path::PathBuf;

pub fn set(key: &str, value: &str) {
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

#[cfg(not(arget_arch = "wasm32"))]
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
