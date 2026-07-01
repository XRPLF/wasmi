//! End-to-end test of the store constructor through the exported C ABI.
//!
//! This deliberately does NOT call the Rust wrapper (`Option<Box<_>>`, `&engine`).
//! Instead it declares the functions with the exact signatures a C caller uses —
//! opaque struct pointers and a raw pointer return whose `NULL` value is how the
//! `None` case surfaces to C — and links against the exported `#[no_mangle]`
//! symbols. That exercises the real symbol names, calling convention, and the
//! null-pointer representation, i.e. what a C program actually sees.

// Force the crate under test to be linked so its exported `#[no_mangle]` C
// symbols are available to the `extern "C"` block below. (This test intentionally
// uses none of the crate's Rust API, so without this the rlib isn't linked in.)
extern crate wasmi_c_api;

use core::ffi::c_void;

// Opaque, C-compatible handles (the standard bindgen idiom): the test only ever
// holds pointers to these, exactly like C.
#[repr(C)]
struct wasm_engine_t {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<c_void>,
}
#[repr(C)]
struct wasm_store_t {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<c_void>,
}

extern "C" {
    fn wasm_engine_new() -> *mut wasm_engine_t;
    fn wasm_engine_delete(engine: *mut wasm_engine_t);
    fn wasm_store_new_with_memory_max_pages(
        engine: *const wasm_engine_t,
        max_pages: u32,
    ) -> *mut wasm_store_t;
    fn wasm_store_delete(store: *mut wasm_store_t);
}

#[test]
fn c_abi_max_pages_null_boundary() {
    unsafe {
        let engine = wasm_engine_new();
        assert!(!engine.is_null(), "engine creation failed");

        // Within the cap (including the 1024-page / 64MB boundary): non-NULL.
        for max_pages in [0u32, 128, 1024] {
            let store = wasm_store_new_with_memory_max_pages(engine, max_pages);
            assert!(
                !store.is_null(),
                "max_pages = {max_pages} should create a store"
            );
            wasm_store_delete(store);
        }

        // Above the cap: the C caller gets NULL, not a process abort.
        for max_pages in [1025u32, u32::MAX] {
            let store = wasm_store_new_with_memory_max_pages(engine, max_pages);
            assert!(
                store.is_null(),
                "max_pages = {max_pages} should return NULL"
            );
        }

        wasm_engine_delete(engine);
    }
}
