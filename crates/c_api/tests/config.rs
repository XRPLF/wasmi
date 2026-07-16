//! C-ABI test for the `start_fn` config policy: disabling it must make module
//! creation reject any module that declares a `start` function.

// Force the crate under test to be linked so its exported C symbols resolve.
extern crate wasmi_c_api;

use core::{ffi::c_void, marker::PhantomData};

macro_rules! opaque {
    ($($name:ident),* $(,)?) => {$(
        #[repr(C)]
        struct $name {
            _opaque: [u8; 0],
            _marker: PhantomData<c_void>,
        }
    )*};
}
opaque!(wasm_config_t, wasm_engine_t, wasm_store_t, wasm_module_t);

#[repr(C)]
struct wasm_byte_vec_t {
    size: usize,
    data: *mut u8,
}

extern "C" {
    fn wasm_config_new() -> *mut wasm_config_t;
    fn wasmi_config_start_fn_set(config: *mut wasm_config_t, enable: bool);
    fn wasm_engine_new_with_config(config: *mut wasm_config_t) -> *mut wasm_engine_t;
    fn wasm_engine_delete(engine: *mut wasm_engine_t);
    fn wasm_store_new_with_memory_max_pages(
        engine: *const wasm_engine_t,
        max_pages: u32,
    ) -> *mut wasm_store_t;
    fn wasm_store_delete(store: *mut wasm_store_t);
    fn wasm_module_new(
        store: *mut wasm_store_t,
        binary: *const wasm_byte_vec_t,
    ) -> *mut wasm_module_t;
    fn wasm_module_delete(module: *mut wasm_module_t);
}

// (module (func $f) (start $f))  -- declares a start function
const MOD_START: [u8; 27] = [
    0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, // header
    0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // type () -> ()
    0x03, 0x02, 0x01, 0x00, // func 0 : type 0
    0x08, 0x01, 0x00, // start section -> func 0
    0x0A, 0x04, 0x01, 0x02, 0x00, 0x0B, // code: (func)
];

// (module (func))  -- no start section
const MOD_NO_START: [u8; 24] = [
    0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, // header
    0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // type () -> ()
    0x03, 0x02, 0x01, 0x00, // func 0 : type 0
    0x0A, 0x04, 0x01, 0x02, 0x00, 0x0B, // code: (func)
];

/// Returns whether `wasm` compiles with `start_fn` set to `allow_start`.
fn module_compiles(allow_start: bool, wasm: &[u8]) -> bool {
    unsafe {
        let cfg = wasm_config_new();
        wasmi_config_start_fn_set(cfg, allow_start);
        let engine = wasm_engine_new_with_config(cfg);
        let store = wasm_store_new_with_memory_max_pages(engine, 128);

        let bin = wasm_byte_vec_t {
            size: wasm.len(),
            data: wasm.as_ptr() as *mut u8,
        };
        let m = wasm_module_new(store, &bin);
        let compiled = !m.is_null();

        if compiled {
            wasm_module_delete(m);
        }
        wasm_store_delete(store);
        wasm_engine_delete(engine);
        compiled
    }
}

#[test]
fn start_fn_config_gates_module_with_start() {
    // Default / allowed: a module declaring a `start` function is accepted.
    assert!(
        module_compiles(true, &MOD_START),
        "start_fn=true should accept a module with a start function",
    );
    // Disabled: the same module is rejected at creation.
    assert!(
        !module_compiles(false, &MOD_START),
        "start_fn=false should reject a module with a start function",
    );
    // Disabled must reject ONLY start-bearing modules: a module without a start
    // section must still compile.
    assert!(
        module_compiles(false, &MOD_NO_START),
        "start_fn=false should still accept a module without a start function",
    );
}
