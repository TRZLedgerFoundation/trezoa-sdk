#pragma once
/**
 * @brief Trezoa logging utilities
 */

#include <trz/types.h>
#include <trz/string.h>
#include <trz/entrypoint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Prints a string to stdout
 */
@SYSCALL void trz_log_(const char *, uint64_t);
#define trz_log(message) trz_log_(message, trz_strlen(message))

/**
 * Prints a 64 bit values represented in hexadecimal to stdout
 */
@SYSCALL void trz_log_64_(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
#define trz_log_64 trz_log_64_

/**
 * Prints the current compute unit consumption to stdout
 */
@SYSCALL void trz_log_compute_units_();
#define trz_log_compute_units() trz_log_compute_units_()

/**
 * Prints the hexadecimal representation of an array
 *
 * @param array The array to print
 */
static void trz_log_array(const uint8_t *array, int len) {
  for (int j = 0; j < len; j++) {
    trz_log_64(0, 0, 0, j, array[j]);
  }
}

/**
 * Print the base64 representation of some arrays.
 */
@SYSCALL void trz_log_data(TrzBytes *, uint64_t);

/**
 * Prints the program's input parameters
 *
 * @param params Pointer to a TrzParameters structure
 */
static void trz_log_params(const TrzParameters *params) {
  trz_log("- Program identifier:");
  trz_log_pubkey(params->program_id);

  trz_log("- Number of KeyedAccounts");
  trz_log_64(0, 0, 0, 0, params->ka_num);
  for (int i = 0; i < params->ka_num; i++) {
    trz_log("  - Is signer");
    trz_log_64(0, 0, 0, 0, params->ka[i].is_signer);
    trz_log("  - Is writable");
    trz_log_64(0, 0, 0, 0, params->ka[i].is_writable);
    trz_log("  - Key");
    trz_log_pubkey(params->ka[i].key);
    trz_log("  - Lamports");
    trz_log_64(0, 0, 0, 0, *params->ka[i].lamports);
    trz_log("  - data");
    trz_log_array(params->ka[i].data, params->ka[i].data_len);
    trz_log("  - Owner");
    trz_log_pubkey(params->ka[i].owner);
    trz_log("  - Executable");
    trz_log_64(0, 0, 0, 0, params->ka[i].executable);
    trz_log("  - Rent Epoch");
    trz_log_64(0, 0, 0, 0, params->ka[i].rent_epoch);
  }
  trz_log("- Instruction data\0");
  trz_log_array(params->data, params->data_len);
}

#ifdef TRZ_TEST
/**
 * Stub functions when building tests
 */
#include <stdio.h>

void trz_log_(const char *s, uint64_t len) {
  printf("Program log: %s\n", s);
}
void trz_log_64(uint64_t arg1, uint64_t arg2, uint64_t arg3, uint64_t arg4, uint64_t arg5) {
  printf("Program log: %llu, %llu, %llu, %llu, %llu\n", arg1, arg2, arg3, arg4, arg5);
}

void trz_log_compute_units_() {
  printf("Program consumption: __ units remaining\n");
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
