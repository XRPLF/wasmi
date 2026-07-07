use crate::{wasm_engine_t, wasmi_error_t, ForeignData};
use alloc::{boxed::Box, sync::Arc};
use core::{cell::UnsafeCell, ffi};
use wasmi::{AsContext, AsContextMut, Store, StoreContext, StoreContextMut, StoreLimits, StoreLimitsBuilder};

/// This representation of a `Store` is used to implement the `wasm.h` API (and
/// *not* the `wasmi.h` API!)
///
/// This is stored alongside `Func` and such for `wasm.h` so each object is
/// independently owned. The usage of `Arc` here is mostly to just get it to be
/// safe to drop across multiple threads, but otherwise acquiring the `context`
/// values from this struct is considered unsafe due to it being unknown how the
/// aliasing is working on the C side of things.
///
/// The aliasing requirements are documented in the C API `wasm.h` itself (at
/// least Wasmi's implementation).
#[derive(Clone)]
pub struct WasmStoreRef {
    inner: Arc<UnsafeCell<Store<StoreLimits>>>,
}

impl WasmStoreRef {
    /// Returns shared access to the store context of the [`WasmStoreRef`].
    ///
    /// Wraps [`wasmi::AsContext`].
    ///
    /// # Safety
    ///
    /// It is the callers responsibility to provide a valid `self`.
    pub unsafe fn context(&self) -> StoreContext<'_, StoreLimits> {
        (*self.inner.get()).as_context()
    }

    /// Returns mutable access to the store context of the [`WasmStoreRef`].
    ///
    /// Wraps [`wasmi::AsContextMut`].
    ///
    /// # Safety
    ///
    /// It is the callers responsibility to provide a valid `self`.
    pub unsafe fn context_mut(&mut self) -> StoreContextMut<'_, StoreLimits> {
        (*self.inner.get()).as_context_mut()
    }
}

/// The Wasm store.
///
/// The returned [`wasm_engine_t`] must be freed using [`wasm_store_delete`].
///
/// Wraps [`wasmi::Store<()>`](wasmi::Store).
#[repr(C)]
#[derive(Clone)]
pub struct wasm_store_t {
    pub(crate) inner: WasmStoreRef,
}

wasmi_c_api_macros::declare_own!(wasm_store_t);

/// Creates a new [`Store<StoreLimits>`](wasmi::Store) for the given `engine`.
///
/// The store is created with no resource limits (original behavior).
/// For memory-limited stores, use [`wasm_store_new_with_memory_max_pages`].
///
/// The returned [`wasm_store_t`] must be freed using [`wasm_store_delete`].
///
/// Wraps [`<wasmi::Store<StoreLimits>>::new`](wasmi::Store::new).
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[allow(clippy::arc_with_non_send_sync)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_store_new(engine: &wasm_engine_t) -> Box<wasm_store_t> {
    let engine = &engine.inner;

    // Create store with no resource limits (original behavior)
    let limits = StoreLimitsBuilder::new().build();
    let store = Store::new(engine, limits);

    Box::new(wasm_store_t {
        inner: WasmStoreRef {
            inner: Arc::new(UnsafeCell::new(store)),
        },
    })
}

/// Creates a new [`Store<StoreLimits>`](wasmi::Store) for the given `engine` with memory limits.
///
/// This function creates a store with resource limits suitable for blockchain smart contracts.
/// The memory limit is enforced during WebAssembly execution.
///
/// If `max_pages` exceeds 1024 (64MB), this function returns `NULL`.
///
/// # Note
///
/// Returning `NULL` (instead of panicking) is deliberate: this function is part
/// of the `extern "C"` boundary, and a panic here would unwind out of `extern
/// "C"` and abort the whole process. For a consensus binary an out-of-range
/// argument must be a recoverable `NULL`, never a validator crash.
///
/// The returned [`wasm_store_t`] must be freed using [`wasm_store_delete`].
///
/// Wraps [`<wasmi::Store<StoreLimits>>::new`](wasmi::Store::new).
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[allow(clippy::arc_with_non_send_sync)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_store_new_with_memory_max_pages(
    engine: &wasm_engine_t,
    max_pages: u32,
) -> Option<Box<wasm_store_t>> {
    // Validate max_pages limit (64MB = 1024 pages).
    if max_pages > 1024 {
        return None;
    }

    // Convert pages to bytes (each page is 64KB)
    let max_memory_bytes = (max_pages as usize) * (64 * 1024);

    // Create store limits with blockchain-suitable defaults
    let limits = StoreLimitsBuilder::new()
        .memory_size(max_memory_bytes) // User-specified memory limit
        .instances(1) // Single instance for blockchain
        .tables(1) // Single table for blockchain
        .memories(1) // Single memory for blockchain
        .table_elements(64) // Limited table elements for blockchain
        .trap_on_grow_failure(false) // Return -1 on growth failure instead of trapping
        .build();

    let mut store = Store::new(&engine.inner, limits);

    // Install the resource limiter
    store.limiter(|limits| limits);

    Some(Box::new(wasm_store_t {
        inner: WasmStoreRef {
            inner: Arc::new(UnsafeCell::new(store)),
        },
    }))
}

/// The Wasm store with foreign data and optional WASI support.
///
/// The returned [`wasm_engine_t`] must be freed using [`wasm_store_delete`].
///
/// Wraps [`wasmi::Store<WasmiStoreData>`](wasmi::Store).
#[repr(C)]
pub struct wasmi_store_t {
    pub(crate) store: Store<WasmiStoreData>,
}

wasmi_c_api_macros::declare_own!(wasmi_store_t);

/// Extensional data stored by [`wasmi_store_t`] to handle foreign data and optional WASI support.
pub struct WasmiStoreData {
    foreign: ForeignData,
}

/// Creates a new [`Store<()>`](wasmi::Store) for the given `engine`.
///
/// - This takes a foreign `data` with an associated `finalizer`.
/// - The returned [`wasm_store_t`] must be freed using [`wasm_store_delete`].
///
/// Wraps [`<wasmi::Store<()>>::new`](wasmi::Store::new).
#[no_mangle]
pub extern "C" fn wasmi_store_new(
    engine: &wasm_engine_t,
    data: *mut ffi::c_void,
    finalizer: Option<extern "C" fn(*mut ffi::c_void)>,
) -> Box<wasmi_store_t> {
    Box::new(wasmi_store_t {
        store: Store::new(
            &engine.inner,
            WasmiStoreData {
                foreign: ForeignData { data, finalizer },
            },
        ),
    })
}

/// Returns mutable access to the store context of the [`wasmi_store_t`].
///
/// Wraps [`wasmi::AsContext`].
///
/// # Safety
///
/// It is the callers responsibility to provide a valid `self`.
#[no_mangle]
pub extern "C" fn wasmi_store_context(
    store: &mut wasmi_store_t,
) -> StoreContextMut<'_, WasmiStoreData> {
    store.store.as_context_mut()
}

/// Returns a pointer to the foreign data of the Wasmi store context.
#[no_mangle]
pub extern "C" fn wasmi_context_get_data(
    store: StoreContext<'_, WasmiStoreData>,
) -> *mut ffi::c_void {
    store.data().foreign.data
}

/// Sets the foreign data of the Wasmi store context.
#[no_mangle]
pub extern "C" fn wasmi_context_set_data(
    mut store: StoreContextMut<'_, WasmiStoreData>,
    data: *mut ffi::c_void,
) {
    store.data_mut().foreign.data = data;
}

/// Returns the current fuel of the Wasmi store context in `fuel`.
///
/// Wraps [`Store::get_fuel`].
///
/// # Errors
///
/// If [`Store::get_fuel`] errors.
#[no_mangle]
pub extern "C" fn wasmi_context_get_fuel(
    store: StoreContext<'_, WasmiStoreData>,
    fuel: &mut u64,
) -> Option<Box<wasmi_error_t>> {
    crate::handle_result(store.get_fuel(), |amt| {
        *fuel = amt;
    })
}

/// Sets the current fuel of the Wasmi store context to `fuel`.
///
/// Wraps [`Store::set_fuel`].
///
/// # Errors
///
/// If [`Store::set_fuel`] errors.
#[no_mangle]
pub extern "C" fn wasmi_context_set_fuel(
    mut store: StoreContextMut<'_, WasmiStoreData>,
    fuel: u64,
) -> Option<Box<wasmi_error_t>> {
    crate::handle_result(store.set_fuel(fuel), |()| {})
}

////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////

/// Returns the current fuel of the wasm store context in `fuel`.
///
/// Wraps [`Store::get_fuel`].
///
/// # Errors
///
/// If [`Store::get_fuel`] errors.
#[no_mangle]
pub extern "C" fn wasm_store_get_fuel(
    store: &wasm_store_t,
    fuel: &mut u64,
) -> Option<Box<wasmi_error_t>> {
    let context = unsafe { store.inner.context() };
    crate::handle_result(context.get_fuel(), |amt| {
        *fuel = amt;
    })
}

/// Sets the current fuel of the wasm store context to `fuel`.
///
/// Wraps [`Store::set_fuel`].
///
/// # Errors
///
/// If [`Store::set_fuel`] errors.
#[no_mangle]
pub extern "C" fn wasm_store_set_fuel(
    store: &mut wasm_store_t,
    fuel: u64,
) -> Option<Box<wasmi_error_t>> {
    let mut context = unsafe { store.inner.context_mut() };
    crate::handle_result(context.set_fuel(fuel), |()| {})
}
