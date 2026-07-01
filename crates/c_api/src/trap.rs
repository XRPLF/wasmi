use crate::{wasm_frame_t, wasm_frame_vec_t, wasm_name_t, wasm_store_t};
use alloc::{boxed::Box, format, string::String, vec::Vec};
use core::{ffi, fmt};
use wasmi::{errors::HostError, Error};

/// A Wasm trap.
///
/// Wraps [`Error`].
#[repr(C)]
pub struct wasm_trap_t {
    pub(crate) error: Error,
}

impl Clone for wasm_trap_t {
    fn clone(&self) -> wasm_trap_t {
        // Note: This API is only needed for the `wasm_trap_copy` API in the C-API.
        //
        // # Note
        //
        // For now the impl here is "fake it til you make it" since this is losing
        // context by only cloning the error string.
        wasm_trap_t {
            error: Error::new(format!("{}", self.error)),
        }
    }
}

wasmi_c_api_macros::declare_ref!(wasm_trap_t);

impl wasm_trap_t {
    /// Creates a [`wasm_trap_t`] from the given [`Error`].
    pub(crate) fn new(error: Error) -> wasm_trap_t {
        wasm_trap_t { error }
    }
}

/// A Wasm error message string buffer.
pub type wasm_message_t = wasm_name_t;

/// Creates a new [`wasm_trap_t`] for the [`wasm_store_t`] with the given `message`.
///
/// # Note
///
/// The `message` is expected to contain a valid null-terminated C string.
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_trap_new(
    _store: &wasm_store_t,
    message: &wasm_message_t,
) -> Box<wasm_trap_t> {
    let message = message.as_slice();
    if message[message.len() - 1] != 0 {
        panic!("wasm_trap_new: expected `message` to be a null-terminated C-string");
    }
    let message = String::from_utf8_lossy(&message[..message.len() - 1]);
    Box::new(wasm_trap_t {
        error: Error::new(message.into_owned()),
    })
}

/// Creates a new [`wasm_trap_t`] from the given `message` and `len` pair.
///
/// # Safety
///
/// The caller is responsible to provide a valid `message` and `len` pair.
#[no_mangle]
pub unsafe extern "C" fn wasmi_trap_new(message: *const u8, len: usize) -> Box<wasm_trap_t> {
    let bytes = crate::slice_from_raw_parts(message, len);
    let message = String::from_utf8_lossy(bytes);
    Box::new(wasm_trap_t {
        error: Error::new(message.into_owned()),
    })
}

/// Returns the error message of the [`wasm_trap_t`].
///
/// Stores the returned error message in `out`.
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_trap_message(trap: &wasm_trap_t, out: &mut wasm_message_t) {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(format!("{:?}", trap.error).as_bytes());
    buffer.reserve_exact(1);
    buffer.push(0);
    out.set_buffer(buffer.into());
}

/// A host-defined trap error carrying an embedder-specific `code`.
///
/// # Note
///
/// This is used by [`wasmi_trap_new_host_code`] so that host functions can
/// raise traps carrying a numeric code that survives round-tripping through the
/// C-API and can later be read back via [`wasmi_trap_code`]. It is kept
/// separate from the WebAssembly specification's [`TrapCode`](wasmi::TrapCode), so host codes
/// can never collide with spec codes.
#[derive(Debug)]
struct HostTrap {
    code: u32,
    message: String,
}

impl fmt::Display for HostTrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl HostError for HostTrap {}

/// Discriminates the origin of a [`wasm_trap_t`]'s code returned by
/// [`wasmi_trap_code`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum wasmi_trap_kind_t {
    /// A trap code as defined by the WebAssembly specification.
    ///
    /// The associated code is a [`TrapCode`](wasmi::TrapCode) value and thus fits into a `u8`.
    WASMI_TRAP_SPEC = 0,
    /// A host-defined trap code created via [`wasmi_trap_new_host_code`].
    WASMI_TRAP_HOST = 1,
}

/// Creates a new host [`wasm_trap_t`] carrying the embedder-specific `code`.
///
/// The trap is intended to be returned from a host function to signal a
/// host-defined error condition. The `code` is preserved and can be read back
/// via [`wasmi_trap_code`], which will report it with kind
/// [`wasmi_trap_kind_t::WASMI_TRAP_HOST`]. Host codes live in a namespace
/// entirely separate from the WebAssembly specification trap codes, so they
/// can never collide with them regardless of value.
///
/// # Note
///
/// The `message` is expected to contain a valid null-terminated C string, or
/// may be `NULL` for an empty message.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wasmi_trap_new_host_code(
    code: u32,
    message: *const ffi::c_char,
) -> Box<wasm_trap_t> {
    let message = if message.is_null() {
        String::new()
    } else {
        let bytes = unsafe { ffi::CStr::from_ptr(message) };
        String::from_utf8_lossy(bytes.to_bytes()).into_owned()
    };
    Box::new(wasm_trap_t::new(Error::host(HostTrap { code, message })))
}

/// Reads the code of the [`wasm_trap_t`] into `out` and its origin into `kind`.
///
/// Returns `true` if the trap carries a code, in which case `out` and `kind`
/// are written; returns `false` for a trap that only carries a message, in
/// which case `out` and `kind` are left untouched.
///
/// When `kind` is [`wasmi_trap_kind_t::WASMI_TRAP_SPEC`] the code is a
/// WebAssembly specification [`TrapCode`](wasmi::TrapCode) (fits into a `u8`); when it is
/// [`wasmi_trap_kind_t::WASMI_TRAP_HOST`] the code is the raw value passed to
/// [`wasmi_trap_new_host_code`]. The two spaces are distinguished by `kind`, not by
/// the numeric value, so host and spec codes never collide.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wasmi_trap_code(
    trap: &wasm_trap_t,
    out: &mut u32,
    kind: &mut wasmi_trap_kind_t,
) -> bool {
    if let Some(trap_code) = trap.error.as_trap_code() {
        *out = u8::from(trap_code) as u32;
        *kind = wasmi_trap_kind_t::WASMI_TRAP_SPEC;
        return true;
    }
    if let Some(host_trap) = trap.error.downcast_ref::<HostTrap>() {
        *out = host_trap.code;
        *kind = wasmi_trap_kind_t::WASMI_TRAP_HOST;
        return true;
    }
    false
}

/// Returns the origin of the [`wasm_trap_t`] if any.
///
/// # Note
///
/// This API is unsupported and will panic upon use.
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_trap_origin(_raw: &wasm_trap_t) -> Option<Box<wasm_frame_t<'_>>> {
    unimplemented!("wasm_trap_origin")
}

/// Returns the trace of the [`wasm_trap_t`].
///
/// Stores the returned trace in `out`.
///
/// # Note
///
/// This API is unsupported and will panic upon use.
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_trap_trace<'a>(_raw: &'a wasm_trap_t, _out: &mut wasm_frame_vec_t<'a>) {
    unimplemented!("wasm_trap_trace")
}
