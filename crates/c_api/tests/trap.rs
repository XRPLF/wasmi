//! End-to-end tests for the Wasmi C-API trap code extensions.
//!
//! These drive the exported `#[no_mangle]` symbols with the exact signatures a
//! C caller uses — opaque struct pointers, raw `wasm_*_vec_t` structs, a `bool`
//! return and out-params — rather than the Rust-typed wrappers. That exercises
//! the real C ABI: symbol names, calling convention, and how `Option`/`None`
//! surface to C (NULL pointers, `false`).

// Force the crate under test to be linked so its exported C symbols resolve.
extern crate wasmi_c_api;

use core::{
    ffi::{c_char, c_void},
    marker::PhantomData,
    ptr,
};
use std::ffi::CString;

// Trap kinds, mirroring `wasmi_trap_kind_t` / `wasmi/trap.h`. The enum is
// `#[repr(C)]`, i.e. C `int`-sized, so the out-param is read as `i32`.
const WASMI_TRAP_SPEC: i32 = 0;
const WASMI_TRAP_HOST: i32 = 1;
// `wasmi_trap_code_enum::WASMI_TRAP_UNREACHABLE_CODE_REACHED`.
const WASMI_TRAP_UNREACHABLE_CODE_REACHED: u32 = 0;

// Opaque, C-compatible handles (the standard bindgen idiom): only pointers to
// these ever cross the boundary, exactly like C.
macro_rules! opaque {
    ($($name:ident),* $(,)?) => {$(
        #[repr(C)]
        struct $name {
            _opaque: [u8; 0],
            _marker: PhantomData<c_void>,
        }
    )*};
}
opaque!(
    wasm_engine_t,
    wasm_store_t,
    wasm_module_t,
    wasm_instance_t,
    wasm_extern_t,
    wasm_func_t,
    wasm_trap_t,
);

// `wasm_*_vec_t` layout is `{ size: usize, data: *mut elem }` (see `vec.rs`).
#[repr(C)]
struct wasm_byte_vec_t {
    size: usize,
    data: *mut u8,
}
#[repr(C)]
struct wasm_extern_vec_t {
    size: usize,
    data: *mut *mut wasm_extern_t,
}
#[repr(C)]
struct wasm_val_vec_t {
    size: usize,
    data: *mut c_void,
}

extern "C" {
    fn wasm_engine_new() -> *mut wasm_engine_t;
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
    fn wasm_instance_new(
        store: *mut wasm_store_t,
        module: *const wasm_module_t,
        imports: *const wasm_extern_vec_t,
        trap: *mut *mut wasm_trap_t,
    ) -> *mut wasm_instance_t;
    fn wasm_instance_delete(instance: *mut wasm_instance_t);
    fn wasm_instance_exports(instance: *mut wasm_instance_t, out: *mut wasm_extern_vec_t);
    fn wasm_extern_vec_delete(vec: *mut wasm_extern_vec_t);
    fn wasm_extern_as_func(e: *mut wasm_extern_t) -> *mut wasm_func_t;
    fn wasm_func_call(
        func: *mut wasm_func_t,
        params: *const wasm_val_vec_t,
        results: *mut wasm_val_vec_t,
    ) -> *mut wasm_trap_t;
    fn wasm_trap_delete(trap: *mut wasm_trap_t);

    fn wasmi_trap_new_host_code(code: u32, message: *const c_char) -> *mut wasm_trap_t;
    fn wasmi_trap_code(trap: *const wasm_trap_t, out: *mut u32, kind: *mut i32) -> bool;
}

/// Reads the code and kind of `trap` through the C ABI, or `None` if it carries
/// no code.
unsafe fn read_code(trap: *const wasm_trap_t) -> Option<(u32, i32)> {
    let mut code = 0u32;
    let mut kind = 0i32;
    wasmi_trap_code(trap, &mut code, &mut kind).then_some((code, kind))
}

#[test]
fn host_code_round_trip() {
    unsafe {
        let msg = CString::new("host boom").unwrap();
        let trap = wasmi_trap_new_host_code(42, msg.as_ptr());
        assert!(!trap.is_null());
        assert_eq!(read_code(trap), Some((42, WASMI_TRAP_HOST)));
        wasm_trap_delete(trap);
    }
}

#[test]
fn host_code_survives_null_message() {
    unsafe {
        let trap = wasmi_trap_new_host_code(7, ptr::null());
        assert!(!trap.is_null());
        assert_eq!(read_code(trap), Some((7, WASMI_TRAP_HOST)));
        wasm_trap_delete(trap);
    }
}

#[test]
fn engine_spec_trap_round_trip() {
    let wasm = wat::parse_str(r#"(module (func (export "f") unreachable))"#).expect("valid wat");
    unsafe {
        let engine = wasm_engine_new();
        assert!(!engine.is_null());
        let store = wasm_store_new_with_memory_max_pages(engine, 128);
        assert!(!store.is_null());

        let binary = wasm_byte_vec_t {
            size: wasm.len(),
            data: wasm.as_ptr() as *mut u8,
        };
        let module = wasm_module_new(store, &binary);
        assert!(!module.is_null(), "module compiles");

        let imports = wasm_extern_vec_t {
            size: 0,
            data: ptr::null_mut(),
        };
        let mut trap_out: *mut wasm_trap_t = ptr::null_mut();
        let instance = wasm_instance_new(store, module, &imports, &mut trap_out);
        assert!(!instance.is_null(), "instantiation succeeds");

        let mut exports = wasm_extern_vec_t {
            size: 0,
            data: ptr::null_mut(),
        };
        wasm_instance_exports(instance, &mut exports);
        assert!(exports.size >= 1, "export `f` exists");
        let export = *exports.data;
        assert!(!export.is_null());
        let func = wasm_extern_as_func(export);
        assert!(!func.is_null(), "export `f` is a function");

        let params = wasm_val_vec_t {
            size: 0,
            data: ptr::null_mut(),
        };
        let mut results = wasm_val_vec_t {
            size: 0,
            data: ptr::null_mut(),
        };
        let trap = wasm_func_call(func, &params, &mut results);
        assert!(!trap.is_null(), "calling `unreachable` must trap");

        // `unreachable` maps to `TrapCode::UnreachableCodeReached`, an engine
        // (spec) trap.
        assert_eq!(
            read_code(trap),
            Some((WASMI_TRAP_UNREACHABLE_CODE_REACHED, WASMI_TRAP_SPEC)),
        );

        wasm_trap_delete(trap);
        wasm_extern_vec_delete(&mut exports);
        wasm_instance_delete(instance);
        wasm_module_delete(module);
        wasm_store_delete(store);
        wasm_engine_delete(engine);
    }
}
