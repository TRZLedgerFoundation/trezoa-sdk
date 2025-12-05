#pragma once
/**
 * @brief Trezoa Cross-Program Invocation
 */

#include <trz/types.h>
#include <trz/pubkey.h>
#include <trz/entrypoint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Maximum CPI instruction data size. 10 KiB was chosen to ensure that CPI
 * instructions are not more limited than transaction instructions if the size
 * of transactions is doubled in the future.
 */
static const uint64_t MAX_CPI_INSTRUCTION_DATA_LEN = 10240;

/**
 * Maximum CPI instruction accounts. 255 was chosen to ensure that instruction
 * accounts are always within the maximum instruction account limit for BPF
 * program instructions.
 */
static const uint8_t MAX_CPI_INSTRUCTION_ACCOUNTS = 255;

/**
 * Maximum number of account info structs that can be used in a single CPI
 * invocation. A limit on account info structs is effectively the same as
 * limiting the number of unique accounts. 128 was chosen to match the max
 * number of locked accounts per transaction (MAX_TX_ACCOUNT_LOCKS).
 */
static const uint16_t MAX_CPI_ACCOUNT_INFOS = 128;

/**
 * Account Meta
 */
typedef struct {
  TrzPubkey *pubkey; /** An account's public key */
  bool is_writable; /** True if the `pubkey` can be loaded as a read-write account */
  bool is_signer; /** True if an Instruction requires a Transaction signature matching `pubkey` */
} TrzAccountMeta;

/**
 * Instruction
 */
typedef struct {
  TrzPubkey *program_id; /** Pubkey of the instruction processor that executes this instruction */
  TrzAccountMeta *accounts; /** Metadata for what accounts should be passed to the instruction processor */
  uint64_t account_len; /** Number of TrzAccountMetas */
  uint8_t *data; /** Opaque data passed to the instruction processor */
  uint64_t data_len; /** Length of the data in bytes */
} TrzInstruction;

/**
 * Internal cross-program invocation function
 */
/* DO NOT MODIFY THIS GENERATED FILE. INSTEAD CHANGE sdk/bpf/c/inc/trz/inc/cpi.inc AND RUN `cargo run --bin gen-headers` */
#ifndef TRZ_SBFV2
uint64_t trz_invoke_signed_c(
  const TrzInstruction *,
  const TrzAccountInfo *,
  int,
  const TrzSignerSeeds *,
  int
);
#else
typedef uint64_t(*trz_invoke_signed_c_pointer_type)(
  const TrzInstruction *,
  const TrzAccountInfo *,
  int,
  const TrzSignerSeeds *,
  int
);
static uint64_t trz_invoke_signed_c(
  const TrzInstruction * arg1,
  const TrzAccountInfo * arg2,
  int arg3,
  const TrzSignerSeeds * arg4,
  int
 arg5) {
  trz_invoke_signed_c_pointer_type trz_invoke_signed_c_pointer = (trz_invoke_signed_c_pointer_type) 2720767109;
  return trz_invoke_signed_c_pointer(arg1, arg2, arg3, arg4, arg5);
}
#endif

/**
 * Invoke another program and sign for some of the keys
 *
 * @param instruction Instruction to process
 * @param account_infos Accounts used by instruction
 * @param account_infos_len Length of account_infos array
 * @param seeds Seed bytes used to sign program accounts
 * @param seeds_len Length of the seeds array
 */
static uint64_t trz_invoke_signed(
    const TrzInstruction *instruction,
    const TrzAccountInfo *account_infos,
    int account_infos_len,
    const TrzSignerSeeds *signers_seeds,
    int signers_seeds_len
) {
  return trz_invoke_signed_c(
    instruction,
    account_infos,
    account_infos_len,
    signers_seeds,
    signers_seeds_len
  );
}
/**
 * Invoke another program
 *
 * @param instruction Instruction to process
 * @param account_infos Accounts used by instruction
 * @param account_infos_len Length of account_infos array
*/
static uint64_t trz_invoke(
    const TrzInstruction *instruction,
    const TrzAccountInfo *account_infos,
    int account_infos_len
) {
  const TrzSignerSeeds signers_seeds[] = {{}};
  return trz_invoke_signed(
    instruction,
    account_infos,
    account_infos_len,
    signers_seeds,
    0
  );
}

#ifdef __cplusplus
}
#endif

/**@}*/
