#pragma once
/**
 * @brief Trezoa Last Restart Slot system call
 */

#include <trz/types.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get Last Restart Slot
 */
/* DO NOT MODIFY THIS GENERATED FILE. INSTEAD CHANGE sdk/sbf/c/inc/trz/inc/last_restart_slot.inc AND RUN `cargo run --bin gen-headers` */
#ifndef TRZ_SBFV2
u64 trz_get_last_restart_slot(uint8_t *result);
#else
typedef u64(*trz_get_last_restart_slot_pointer_type)(uint8_t *result);
static u64 trz_get_last_restart_slot(uint8_t *result arg1) {
  trz_get_last_restart_slot_pointer_type trz_get_last_restart_slot_pointer = (trz_get_last_restart_slot_pointer_type) 411697201;
  return trz_get_last_restart_slot_pointer(arg1);
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
