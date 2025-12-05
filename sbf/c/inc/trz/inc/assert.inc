#pragma once
/**
 * @brief Trezoa assert and panic utilities
 */

#include <trz/types.h>

#ifdef __cplusplus
extern "C" {
#endif


/**
 * Panics
 *
 * Prints the line number where the panic occurred and then causes
 * the SBF VM to immediately halt execution. No accounts' data are updated
 */
@SYSCALL void trz_panic_(const char *, uint64_t, uint64_t, uint64_t);
#define trz_panic() trz_panic_(__FILE__, sizeof(__FILE__), __LINE__, 0)

/**
 * Asserts
 */
#define trz_assert(expr)  \
if (!(expr)) {          \
  trz_panic(); \
}

#ifdef TRZ_TEST
/**
 * Stub functions when building tests
 */
#include <stdio.h>
#include <stdlib.h>

void trz_panic_(const char *file, uint64_t len, uint64_t line, uint64_t column) {
  printf("Panic in %s at %d:%d\n", file, line, column);
  abort();
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
