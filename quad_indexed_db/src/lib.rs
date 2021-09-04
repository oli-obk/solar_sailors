//! Provides access to the browser's local IndexedDB.
//!
//! Add file [`quad_indexed_db/js/quad-indexed-db.js`](https://github.com/oli-obk/solar_sailors/blob/main/quad_indexed_db/quad-indexed_db.js) to your project.
//!
//! Add file [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/master/js/sapp_jsutils.js) file to your project.
//!
//! Add this lines after loading of `gl.js` and before loading of your wasm in your `index.html`:
//! ```html
//! <script src="sapp_jsutils.js"></script>
//! <script src="quad-indexed-db.js"></script>
//! ```
//! Done! Now you can use this crate.

use std::{future::Future, task::{Poll, Waker}};

use sapp_jsutils::{JsObject, JsObjectWeak};

#[no_mangle]
extern "C" fn quad_indexed_db_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

#[no_mangle]
extern "C" fn quad_indexed_db_wake(waker: Box<Waker>) {
    waker.wake();
}

#[no_mangle]
extern "C" fn quad_indexed_db_clone_waker(waker: &Waker) -> Box<Waker> {
    Box::new(waker.clone())
}

extern "C" {
    fn quad_indexed_db_start_transaction() -> bool;
    #[allow(improper_ctypes)]
    fn quad_indexed_db_finish_transaction(waker: &Waker) -> bool;
    fn quad_indexed_db_get(key: JsObjectWeak) -> JsObject;
    fn quad_indexed_db_set(key: JsObjectWeak, value: JsObjectWeak);
    fn quad_indexed_db_remove(key: JsObjectWeak);
    fn quad_indexed_db_clear();
}

fn js_to_string(object: JsObject) -> String {
    let mut result = String::new();
    object.to_string(&mut result);
    result
}

/// Get entry by key, if any.
pub fn get(key: &str) -> impl Future<Output = Option<String>> {
    struct CheckDone {
        id: JsObject,
    }
    impl Future for CheckDone {
        type Output = Option<String>;

        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<Self::Output> {
            if self.id.field_u32("done") == 0 {
                self.id.set_field_u32("waker", Box::into_raw(Box::new(cx.waker().clone())) as usize as u32);
                Poll::Pending
            } else {
                let field = self.id.field("value");
                Poll::Ready((!field.is_undefined()).then(|| js_to_string(field)))
            }
        }
    }
    CheckDone {
        id: unsafe { quad_indexed_db_get(JsObject::string(key).weak()) },
    }
}

pub fn set(key: &str, value: &str) {
    unsafe {
        quad_indexed_db_set(JsObject::string(key).weak(), JsObject::string(value).weak());
    }
}

/// Remove entry from the local indexed_db.
pub fn remove(key: &str) {
    unsafe {
        quad_indexed_db_remove(JsObject::string(key).weak());
    }
}

/// Remove all entries from local indexed_db.
pub fn clear() {
    unsafe {
        quad_indexed_db_clear();
    }
}

pub async fn transaction<FUT:Future<Output = ()>>(f: impl FnOnce() -> FUT) {
    struct CheckDone;
    impl Future for CheckDone {
        type Output = ();

        fn poll(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<Self::Output> {
            if unsafe { quad_indexed_db_finish_transaction(cx.waker()) } {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }
    assert!(unsafe { quad_indexed_db_start_transaction() });
    f().await;
    CheckDone.await
}
