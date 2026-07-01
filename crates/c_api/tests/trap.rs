//! Round-trip tests for the Wasmi C-API trap code extensions.

use core::ptr;
use std::ffi::CString;
use wasmi_c_api::{
    wasm_byte_vec_t,
    wasm_engine_new,
    wasm_extern_as_func,
    wasm_extern_vec_t,
    wasm_func_call,
    wasm_instance_exports,
    wasm_instance_new,
    wasm_module_new,
    wasm_store_new,
    wasm_trap_t,
    wasm_val_vec_t,
    wasmi_trap_code,
    wasmi_trap_kind_t,
    wasmi_trap_new_host_code,
};

/// Reads the code and kind of `trap`, returning `None` if it carries no code.
fn read_code(trap: &wasm_trap_t) -> Option<(u32, wasmi_trap_kind_t)> {
    let mut code = 0u32;
    let mut kind = wasmi_trap_kind_t::WASMI_TRAP_HOST;
    wasmi_trap_code(trap, &mut code, &mut kind).then_some((code, kind))
}

#[test]
fn host_code_round_trip() {
    let msg = CString::new("host boom").unwrap();
    let trap = wasmi_trap_new_host_code(42, msg.as_ptr());
    assert_eq!(
        read_code(&trap),
        Some((42, wasmi_trap_kind_t::WASMI_TRAP_HOST)),
    );
}

#[test]
fn host_code_survives_null_message() {
    let trap = wasmi_trap_new_host_code(7, ptr::null());
    assert_eq!(
        read_code(&trap),
        Some((7, wasmi_trap_kind_t::WASMI_TRAP_HOST)),
    );
}

/// Runs a module whose exported `f` executes `unreachable` and returns the trap
/// produced by the Wasmi engine.
fn run_trapping_module() -> Box<wasm_trap_t> {
    let wasm = wat::parse_str(r#"(module (func (export "f") unreachable))"#).expect("valid wat");
    unsafe {
        let engine = wasm_engine_new();
        let mut store = wasm_store_new(&engine);

        let binary = wasm_byte_vec_t::from(wasm);
        let module = wasm_module_new(&mut store, &binary).expect("module compiles");

        let imports = wasm_extern_vec_t::from(Vec::new());
        let mut trap_ptr: *mut wasm_trap_t = ptr::null_mut();
        let mut instance = wasm_instance_new(&mut store, &module, &imports, Some(&mut trap_ptr))
            .expect("instantiation succeeds");

        let mut exports = wasm_extern_vec_t::from(Vec::new());
        wasm_instance_exports(&mut instance, &mut exports);
        let mut exports = exports.take();
        let export = exports[0].as_deref_mut().expect("export `f` exists");
        let func = wasm_extern_as_func(export).expect("export `f` is a function");

        let params = wasm_val_vec_t::from(Vec::new());
        let mut results = wasm_val_vec_t::from(Vec::new());
        let trap = wasm_func_call(func, &params, &mut results);
        assert!(!trap.is_null(), "calling `unreachable` must trap");
        Box::from_raw(trap)
    }
}

#[test]
fn engine_spec_trap_round_trip() {
    let trap = run_trapping_module();
    // `unreachable` maps to `TrapCode::UnreachableCodeReached`.
    let expected = wasmi::TrapCode::UnreachableCodeReached as u32;
    assert_eq!(
        read_code(&trap),
        Some((expected, wasmi_trap_kind_t::WASMI_TRAP_SPEC)),
    );
}
