use alloc::boxed::Box;
use wasmi::{CompilationMode, Config, EnforcedLimits};

/// The Wasm configuration.
///
/// Wraps [`wasmi::Config`]
#[repr(C)]
#[derive(Clone)]
pub struct wasm_config_t {
    pub(crate) inner: Config,
}

wasmi_c_api_macros::declare_own!(wasm_config_t);

/// Creates a new default initialized [`wasm_config_t`].
///
/// The returned [`wasm_config_t`] must be freed using [`wasm_config_delete`]
/// or consumed by [`wasm_engine_new_with_config`].
///
/// Wraps [`wasmi::Config::default`].
///
/// [`wasm_engine_new_with_config`]: crate::wasm_engine_new_with_config
#[cfg_attr(not(feature = "prefix-symbols"), no_mangle)]
#[cfg_attr(feature = "prefix-symbols", wasmi_c_api_macros::prefix_symbol)]
pub extern "C" fn wasm_config_new() -> Box<wasm_config_t> {
    Box::new(wasm_config_t {
        inner: Config::default(),
    })
}

/// Enables or disables support for the Wasm [`mutable-global`] proposal.
///
/// Wraps [`wasmi::Config::wasm_multi_value`]
///
/// [`mutable-global`]: <https://github.com/WebAssembly/mutable-global>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_mutable_globals_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_mutable_global(enable);
}

/// Enables or disables support for the Wasm [`multi-value`] proposal.
///
/// Wraps [`wasmi::Config::wasm_multi_value`]
///
/// [`multi-value`]: <https://github.com/WebAssembly/multi-value>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_multi_value_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_multi_value(enable);
}

/// Enables or disables support for the Wasm [`sign-extension-ops`] proposal.
///
/// Wraps [`wasmi::Config::wasm_sign_extension`]
///
/// [`sign-extension-ops`]: <https://github.com/WebAssembly/sign-extension-ops>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_sign_extension_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_sign_extension(enable);
}

/// Enables or disables support for the Wasm [`nontrapping-float-to-int-conversions`] proposal.
///
/// Wraps [`wasmi::Config::wasm_saturating_float_to_int`]
///
/// [`nontrapping-float-to-int-conversions`]: <https://github.com/WebAssembly/nontrapping-float-to-int-conversions>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_saturating_float_to_int_set(
    c: &mut wasm_config_t,
    enable: bool,
) {
    c.inner.wasm_saturating_float_to_int(enable);
}

/// Enables or disables support for the Wasm [`bulk-memory-operations`] proposal.
///
/// Wraps [`wasmi::Config::wasm_bulk_memory`]
///
/// [`bulk-memory-operations`]: <https://github.com/WebAssembly/bulk-memory-operations>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_bulk_memory_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_bulk_memory(enable);
}

/// Enables or disables support for the Wasm [`reference-types`] proposal.
///
/// Wraps [`wasmi::Config::wasm_reference_types`]
///
/// [`reference-types`]: <https://github.com/WebAssembly/reference-types>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_reference_types_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_reference_types(enable);
}

/// Enables or disables support for the Wasm [`tail-call`] proposal.
///
/// Wraps [`wasmi::Config::wasm_tail_call`]
///
/// [`tail-call`]: <https://github.com/WebAssembly/tail-call>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_tail_call_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_tail_call(enable);
}

/// Enables or disables support for the Wasm [`extended-const`] proposal.
///
/// Wraps [`wasmi::Config::wasm_extended_const`]
///
/// [`extended-const`]: <https://github.com/WebAssembly/extended-const>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_extended_const_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_extended_const(enable);
}

/// Enables or disables support for the Wasm [`multi-memory`] proposal.
///
/// Wraps [`wasmi::Config::wasm_multi_memory`]
///
/// [`multi-memory`]: <https://github.com/WebAssembly/multi-memory>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_multi_memory_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_multi_memory(enable);
}

/// Enables or disables support for the Wasm [`custom-page-sizes`] proposal.
///
/// Wraps [`wasmi::Config::wasm_custom_page_sizes`]
///
/// [`custom-page-sizes`]: <https://github.com/WebAssembly/custom-page-sizes>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_custom_page_sizes_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_custom_page_sizes(enable);
}

/// Enables or disables support for the Wasm [`memory64`] proposal.
///
/// Wraps [`wasmi::Config::wasm_memory64`]
///
/// [`memory64`]: <https://github.com/WebAssembly/memory64>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_memory64_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_memory64(enable);
}

/// Enables or disables support for the Wasm [`wide-arithmetic`] proposal.
///
/// Wraps [`wasmi::Config::wasm_wide_arithmetic`]
///
/// [`wide-arithmetic`]: <https://github.com/WebAssembly/wide-arithmetic>
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_wide_arithmetic_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_wide_arithmetic(enable);
}

/// Enables or disables support for the Wasm [`simd`] proposal.
///
/// Wraps [`wasmi::Config::wasm_simd`]
///
/// [`simd`]: <https://github.com/WebAssembly/simd>
#[cfg(feature = "simd")]
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_simd_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_simd(enable);
}

/// Enables or disables support for the Wasm [`relaxed-simd`] proposal.
///
/// Wraps [`wasmi::Config::wasm_relaxed_simd`]
///
/// [`relaxed-simd`]: <https://github.com/WebAssembly/relaxed-simd>
#[cfg(feature = "simd")]
#[no_mangle]
pub extern "C" fn wasmi_config_wasm_relaxed_simd_set(c: &mut wasm_config_t, enable: bool) {
    c.inner.wasm_relaxed_simd(enable);
}

/// Enables or disables support for floating point numbers for the config.
///
/// Wraps [`wasmi::Config::floats`]
#[no_mangle]
pub extern "C" fn wasmi_config_floats_set(config: &mut wasm_config_t, enable: bool) {
    config.inner.floats(enable);
}

/// Enables or disables fuel consumption for the config.
///
/// Wraps [`wasmi::Config::consume_fuel`]
#[no_mangle]
pub extern "C" fn wasmi_config_consume_fuel_set(config: &mut wasm_config_t, enable: bool) {
    config.inner.consume_fuel(enable);
}

/// Compilation modes supported by the Wasmi execution engine.
///
/// Wraps [`wasmi::CompilationMode`]
#[repr(u8)]
#[derive(Clone)]
pub enum wasmi_compilation_mode_t {
    WASMI_COMPILATION_MODE_EAGER,
    WASMI_COMPILATION_MODE_LAZY_TRANSLATION,
    WASMI_COMPILATION_MODE_LAZY,
}

/// Sets the compilation mode for the config.
///
/// Wraps [`wasmi::Config::compilation_mode`]
#[no_mangle]
pub extern "C" fn wasmi_config_compilation_mode_set(
    config: &mut wasm_config_t,
    mode: wasmi_compilation_mode_t,
) {
    use wasmi_compilation_mode_t::*;
    config.inner.compilation_mode(match mode {
        WASMI_COMPILATION_MODE_EAGER => CompilationMode::Eager,
        WASMI_COMPILATION_MODE_LAZY_TRANSLATION => CompilationMode::LazyTranslation,
        WASMI_COMPILATION_MODE_LAZY => CompilationMode::Lazy,
    });
}

/// Enables or disables processing of Wasm custom sections.
///
/// Wraps [`wasmi::Config::ignore_custom_sections`]
#[no_mangle]
pub extern "C" fn wasmi_config_ignore_custom_sections_set(
    config: &mut wasm_config_t,
    enable: bool,
) {
    config.inner.ignore_custom_sections(enable);
}

/// Sets the maximum recursion depth of the engine's stack during execution.
///
/// An execution traps if it exceeds this limit.
///
/// Wraps [`wasmi::Config::set_max_recursion_depth`]
#[no_mangle]
pub extern "C" fn wasmi_config_set_max_recursion_depth(
    config: &mut wasm_config_t,
    value: usize,
) {
    config.inner.set_max_recursion_depth(value);
}

/// Sets the minimum (or initial) height of the engine's value stack in bytes.
///
/// # Note
///
/// - Lower initial heights may improve memory consumption.
/// - Higher initial heights may improve cold start times.
///
/// # Panics
///
/// If `value` is greater than the current maximum height of the value stack.
///
/// Wraps [`wasmi::Config::set_min_stack_height`]
#[no_mangle]
pub extern "C" fn wasmi_config_set_min_stack_height(config: &mut wasm_config_t, value: usize) {
    config.inner.set_min_stack_height(value);
}

/// Sets the maximum height of the engine's value stack in bytes.
///
/// An execution traps if it exceeds this limit.
///
/// # Panics
///
/// If `value` is less than the current minimum height of the value stack.
///
/// Wraps [`wasmi::Config::set_max_stack_height`]
#[no_mangle]
pub extern "C" fn wasmi_config_set_max_stack_height(config: &mut wasm_config_t, value: usize) {
    config.inner.set_max_stack_height(value);
}

/// Sets the maximum number of cached stacks for reuse.
///
/// # Note
///
/// - A higher value may improve execution performance.
/// - A lower value may improve memory consumption.
///
/// Wraps [`wasmi::Config::set_max_cached_stacks`]
#[no_mangle]
pub extern "C" fn wasmi_config_set_max_cached_stacks(config: &mut wasm_config_t, value: usize) {
    config.inner.set_max_cached_stacks(value);
}

/// Enforced limits for Wasm module parsing and compilation.
///
/// Wraps [`wasmi::EnforcedLimits`]
#[repr(C)]
#[derive(Clone)]
pub struct wasmi_enforced_limits_t {
    pub(crate) inner: EnforcedLimits,
}

wasmi_c_api_macros::declare_own!(wasmi_enforced_limits_t);

/// Creates a new [`wasmi_enforced_limits_t`] with strict limits.
///
/// This set of strict enforced rules can be used to safeguard against
/// malicious actors trying to attack the Wasmi compilation procedures.
///
/// The strict limits are:
/// - max_globals: 1000
/// - max_functions: 10,000
/// - max_tables: 100
/// - max_element_segments: 1000
/// - max_memories: 1
/// - max_data_segments: 1000
/// - max_params: 32
/// - max_results: 32
/// - min_avg_bytes_per_function: 40 (enforced at 1000+ total bytes)
///
/// The returned [`wasmi_enforced_limits_t`] must be freed using
/// [`wasmi_enforced_limits_delete`] or consumed by [`wasmi_config_enforced_limits_set`].
///
/// Wraps [`wasmi::EnforcedLimits::strict`]
#[no_mangle]
pub extern "C" fn wasmi_enforced_limits_strict() -> Box<wasmi_enforced_limits_t> {
    Box::new(wasmi_enforced_limits_t {
        inner: EnforcedLimits::strict(),
    })
}

/// Sets the enforced limits for the config.
///
/// By default no limits are enforced.
///
/// Wraps [`wasmi::Config::enforced_limits`]
#[no_mangle]
pub extern "C" fn wasmi_config_enforced_limits_set(
    config: &mut wasm_config_t,
    limits: &wasmi_enforced_limits_t,
) {
    config.inner.enforced_limits(limits.inner);
}
