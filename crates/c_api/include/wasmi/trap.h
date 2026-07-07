/**
 * \file wasmi/trap.h
 *
 * \brief Wasmi-specific extensions to #wasm_trap_t
 */

#ifndef WASMI_TRAP_H
#define WASMI_TRAP_H

#include <stdbool.h>
#include <stdint.h>
#include <wasm.h>

#define own

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief WebAssembly specification trap codes.
 *
 * These are the codes reported by #wasmi_trap_code when the trap kind is
 * #WASMI_TRAP_SPEC. They mirror the WebAssembly specification trap set and
 * always fit into a `uint8_t`.
 */
typedef enum wasmi_trap_code_enum {
  /// Wasm code executed an `unreachable` instruction.
  WASMI_TRAP_UNREACHABLE_CODE_REACHED = 0,
  /// Out of bounds linear memory access.
  WASMI_TRAP_MEMORY_OUT_OF_BOUNDS = 1,
  /// Out of bounds table access.
  WASMI_TRAP_TABLE_OUT_OF_BOUNDS = 2,
  /// `call_indirect` to a null table element.
  WASMI_TRAP_INDIRECT_CALL_TO_NULL = 3,
  /// Integer division by zero.
  WASMI_TRAP_INTEGER_DIVISION_BY_ZERO = 4,
  /// Integer arithmetic overflow.
  WASMI_TRAP_INTEGER_OVERFLOW = 5,
  /// Invalid conversion to an integer type.
  WASMI_TRAP_BAD_CONVERSION_TO_INTEGER = 6,
  /// Call stack exhausted (stack overflow).
  WASMI_TRAP_STACK_OVERFLOW = 7,
  /// Indirect call type mismatch.
  WASMI_TRAP_BAD_SIGNATURE = 8,
  /// WebAssembly execution ran out of fuel.
  WASMI_TRAP_OUT_OF_FUEL = 9,
  /// A growth operation was refused by an installed resource limiter.
  WASMI_TRAP_GROWTH_OPERATION_LIMITED = 10,
} wasmi_trap_code_enum;

/**
 * \brief Discriminates the origin of a trap code returned by #wasmi_trap_code.
 */
typedef enum wasmi_trap_kind_enum {
  /// The code is a WebAssembly specification trap (#wasmi_trap_code_enum).
  WASMI_TRAP_SPEC = 0,
  /// The code is a host-defined code created via #wasmi_trap_new_host_code.
  WASMI_TRAP_HOST = 1,
} wasmi_trap_kind_t;

/**
 * \brief Creates a new host trap carrying an embedder-specific \p code.
 *
 * The returned trap is intended to be returned from a host function to signal
 * a host-defined error condition. The \p code is preserved and can be read
 * back with #wasmi_trap_code, which reports it with kind #WASMI_TRAP_HOST.
 *
 * Host codes live in a namespace entirely separate from the WebAssembly
 * specification trap codes, so they can never collide with them regardless of
 * value. \p message may be `NULL` for an empty message.
 */
WASM_API_EXTERN own wasm_trap_t *wasmi_trap_new_host_code(uint32_t code,
                                                          const char *message);

/**
 * \brief Reads the trap's code into \p out and its origin into \p kind.
 *
 * Returns `true` if the trap carries a code, writing \p out and \p kind; the
 * value of \p kind indicates whether \p out is a WebAssembly specification
 * code (#WASMI_TRAP_SPEC, see #wasmi_trap_code_enum) or a host-defined code
 * (#WASMI_TRAP_HOST). The two spaces are distinguished by \p kind and never by
 * value, so host and spec codes never collide.
 *
 * Returns `false` for a trap that only carries a message, leaving \p out and
 * \p kind untouched.
 */
WASM_API_EXTERN bool wasmi_trap_code(const wasm_trap_t *trap, uint32_t *out,
                                     wasmi_trap_kind_t *kind);

#ifdef __cplusplus
} // extern "C"
#endif

#undef own

#endif // WASMI_TRAP_H
