use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use scaffold_core::Engine;

pub const OK: i32 = 0;
pub const ERR_INVALID_HANDLE: i32 = -1;
pub const ERR_BAD_ARGUMENT: i32 = -2;
pub const ERR_INTERNAL: i32 = -3;
pub const ABI_VERSION: i32 = 1;

static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(1);
static ENGINES: LazyLock<Mutex<HashMap<usize, Engine>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn guard(f: impl FnOnce() -> i32) -> i32 {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(rc) => rc,
        Err(_) => ERR_INTERNAL,
    }
}

fn guard_handle(f: impl FnOnce() -> usize) -> usize {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_calc_abi_version() -> i32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_calc_add(a: i32, b: i32) -> i32 {
    guard(|| scaffold_core::Calc::add(a, b))
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_calc_engine_create() -> usize {
    guard_handle(|| {
        let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
        ENGINES
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(handle, Engine::new());
        handle
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_calc_engine_set_factor(handle: usize, factor: i32) -> i32 {
    guard(|| {
        let mut engines = ENGINES.lock().unwrap_or_else(|e| e.into_inner());
        match engines.get_mut(&handle) {
            Some(engine) => {
                engine.set_factor(factor);
                OK
            }
            None => ERR_INVALID_HANDLE,
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scaffold_calc_engine_apply(
    handle: usize,
    input: i64,
    out: *mut i64,
) -> i32 {
    guard(|| {
        if out.is_null() {
            return ERR_BAD_ARGUMENT;
        }
        let mut engines = ENGINES.lock().unwrap_or_else(|e| e.into_inner());
        match engines.get_mut(&handle) {
            Some(engine) => {
                let acc = engine.apply(input);
                unsafe { *out = acc };
                OK
            }
            None => ERR_INVALID_HANDLE,
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn scaffold_calc_engine_destroy(handle: usize) -> i32 {
    guard(|| {
        let mut engines = ENGINES.lock().unwrap_or_else(|e| e.into_inner());
        match engines.remove(&handle) {
            Some(_) => OK,
            None => ERR_INVALID_HANDLE,
        }
    })
}
