#pragma once
/**
 * @brief Trezoa bn128 elliptic curve addition, multiplication, and pairing
**/

#include <trz/types.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Addition on elliptic curves alt_bn128
 *
 * @param group_op ...
 * @param input ...
 * @param input_size ...
 * @param result 64 byte array to hold the result. ...
 * @return 0 if executed successfully
 */
/* DO NOT MODIFY THIS GENERATED FILE. INSTEAD CHANGE sdk/sbf/c/inc/trz/inc/alt_bn128.inc AND RUN `cargo run --bin gen-headers` */
#ifndef TRZ_SBFV2
uint64_t trz_alt_bn128(
        const uint64_t *group_op,
        const uint8_t *input,
        const uint64_t input_size,
        uint8_t *result
);
#else
typedef uint64_t(*trz_alt_bn128_pointer_type)(
        const uint64_t *group_op,
        const uint8_t *input,
        const uint64_t input_size,
        uint8_t *result
);
static uint64_t trz_alt_bn128(
        const uint64_t *group_op arg1,
        const uint8_t *input arg2,
        const uint64_t input_size arg3,
        uint8_t *result
 arg4) {
  trz_alt_bn128_pointer_type trz_alt_bn128_pointer = (trz_alt_bn128_pointer_type) 2551807235;
  return trz_alt_bn128_pointer(arg1, arg2, arg3, arg4);
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
