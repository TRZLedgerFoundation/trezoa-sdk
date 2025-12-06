#![cfg(feature = "test-bpf")]

use {
    trezoa_program_test::*,
    trezoa_sbf_rust_sanity::process_instruction,
    trezoa_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
};

#[tokio::test]
async fn test_sanity() {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        "trezoa_sbf_rust_sanity",
        program_id,
        processor!(process_instruction),
    );
    let (mut banks_client, payer_keypair, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &(),
            vec![
                AccountMeta::new(payer_keypair.pubkey(), true),
                AccountMeta::new(Keypair::new().pubkey(), false),
            ],
        )],
        Some(&payer_keypair.pubkey()),
    );
    transaction.sign(&[&payer_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
