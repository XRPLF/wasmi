/**
 * \file wasmi/config.h
 *
 * \brief Wasmi-specific extensions to #wasm_config_t
 */

#ifndef WASMI_CONFIG_H
#define WASMI_CONFIG_H

#include <wasm.h>

#ifdef __cplusplus
extern "C" {
#endif

#define WASMI_CONFIG_PROP(ret, name, ty)                                       \
  WASM_API_EXTERN ret wasmi_config_##name##_set(wasm_config_t *, ty);

/**
 * \brief Whether or not fuel is enabled for generated code.
 *
 * When enabled it will enable fuel counting meaning that fuel will be consumed
 * every time a Wasm instruction is executed, and trap when reaching zero.
 *
 * Default value: `false`
 */
WASMI_CONFIG_PROP(void, consume_fuel, bool)

/**
 * \brief Whether or not to ignore Wasm custom sections.
 *
 * When enabled it will ignoe Wasm custom sections when creating a Wasm module.
 *
 * Default value: `false`
 */
WASMI_CONFIG_PROP(void, ignore_custom_sections, bool)

/**
 * \brief Sets the maximum recursion depth of the engine's stack during execution.
 *
 * An execution traps if it exceeds this limit.
 */
// NOTE: declared explicitly (not via WASMI_CONFIG_PROP): the macro appends a
// `_set` suffix, but the exported symbol is `wasmi_config_set_max_recursion_depth`.
WASM_API_EXTERN void wasmi_config_set_max_recursion_depth(wasm_config_t *, size_t);

/**
 * \brief Sets the minimum (or initial) height of the engine's value stack in bytes.
 *
 * Lower initial heights may improve memory consumption.
 * Higher initial heights may improve cold start times.
 *
 * Note: Panics if value is greater than the current maximum height of the value stack.
 */
WASM_API_EXTERN void wasmi_config_set_min_stack_height(wasm_config_t *, size_t);

/**
 * \brief Sets the maximum height of the engine's value stack in bytes.
 *
 * An execution traps if it exceeds this limit.
 *
 * Note: Panics if value is less than the current minimum height of the value stack.
 */
WASM_API_EXTERN void wasmi_config_set_max_stack_height(wasm_config_t *, size_t);

/**
 * \brief Sets the maximum number of cached stacks for reuse.
 *
 * A higher value may improve execution performance.
 * A lower value may improve memory consumption.
 */
WASMI_CONFIG_PROP(void, set_max_cached_stacks, size_t)

/**
 * \brief Whether or not to Wasm mutable-globals proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_mutable_globals, bool)

/**
 * \brief Whether or not to Wasm multi-value proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_multi_value, bool)

/**
 * \brief Whether or not to Wasm sign-extension proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_sign_extension, bool)

/**
 * \brief Whether or not to Wasm non-trapping-float-to-int-conversions proposal
 * is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_saturating_float_to_int, bool)

/**
 * \brief Whether or not to Wasm bulk-memory-ops proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_bulk_memory, bool)

/**
 * \brief Whether or not to Wasm reference-types proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_reference_types, bool)

/**
 * \brief Whether or not to Wasm tail-call proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_tail_call, bool)

/**
 * \brief Whether or not to Wasm extended-const proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_extended_const, bool)

/**
 * \brief Whether or not to Wasm multi-memory proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_multi_memory, bool)

/**
 * \brief Whether or not to Wasm custom-page-sizes proposal is enabled.
 *
 * Default value: `false`
 */
WASMI_CONFIG_PROP(void, wasm_custom_page_sizes, bool)

/**
 * \brief Whether or not to Wasm memory64 proposal is enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, wasm_memory64, bool)

/**
 * \brief Whether or not to Wasm wide-arithmetic proposal is enabled.
 *
 * Default value: `false`
 */
WASMI_CONFIG_PROP(void, wasm_wide_arithmetic, bool)

/**
 * \brief Whether or not to floating Wasm point types and operations are
 * enabled.
 *
 * Default value: `true`
 */
WASMI_CONFIG_PROP(void, floats, bool)

/**
 * \brief Different ways Wasmi can compile Wasm bytecode into Wasmi bytecode.
 *
 * The default value is #WASMI_COMPILATION_MODE_LAZY_TRANSLATION.
 *
 * \note The enumerator order must match `wasmi::CompilationMode` (and the
 * `wasmi_compilation_mode_t` enum in `config.rs`), since values are passed by
 * their integer discriminant across the C ABI.
 */
enum wasmi_compilation_mode_enum {
  /// Wasmi compiles and validates Wasm bytecode eagerly.
  WASMI_COMPILATION_MODE_EAGER,
  /// Wasmi compiles Wasm bytecode upon first use but validates Wasm bytecode
  /// eagerly.
  WASMI_COMPILATION_MODE_LAZY_TRANSLATION,
  /// Wasmi compiles and validates Wasm bytecode upon first use.
  WASMI_COMPILATION_MODE_LAZY,
};

/**
 * \brief Sets how Wasmi compiles Wasm bytecode; see #wasmi_compilation_mode_enum.
 *
 * Default value: #WASMI_COMPILATION_MODE_LAZY_TRANSLATION.
 */
WASMI_CONFIG_PROP(void, compilation_mode, enum wasmi_compilation_mode_enum)

#undef WASMI_CONFIG_PROP

/**
 * \brief Enforced limits for Wasm module parsing and compilation.
 *
 * Opaque type representing limits that can be enforced on Wasm modules.
 */
typedef struct wasmi_enforced_limits_t wasmi_enforced_limits_t;

/**
 * \brief Creates a new enforced limits object with strict preset values.
 *
 * This set of strict enforced rules can be used to safeguard against
 * malicious actors trying to attack the Wasmi compilation procedures.
 *
 * The strict limits are:
 * - max_globals: 1000
 * - max_functions: 10,000
 * - max_tables: 100
 * - max_element_segments: 1000
 * - max_memories: 1
 * - max_data_segments: 1000
 * - max_params: 32
 * - max_results: 32
 * - min_avg_bytes_per_function: 40 (enforced at 1000+ total bytes)
 *
 * The returned object must be freed using wasmi_enforced_limits_delete().
 *
 * \return A new enforced limits object with strict preset values
 */
WASM_API_EXTERN wasmi_enforced_limits_t* wasmi_enforced_limits_strict();

/**
 * \brief Deletes an enforced limits object.
 *
 * \param limits The enforced limits object to delete
 */
WASM_API_EXTERN void wasmi_enforced_limits_delete(wasmi_enforced_limits_t* limits);

/**
 * \brief Sets the enforced limits for the configuration.
 *
 * By default no limits are enforced. Use this function to apply a set of
 * enforced limits (such as those created by wasmi_enforced_limits_strict())
 * to the configuration.
 *
 * \param config The configuration to modify
 * \param limits The enforced limits to apply
 */
WASM_API_EXTERN void wasmi_config_enforced_limits_set(
    wasm_config_t* config,
    const wasmi_enforced_limits_t* limits
);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WASMI_CONFIG_H
