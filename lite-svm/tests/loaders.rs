use lite_svm::{bank::LiteSVM, deploy_program, deploy_upgradeable_program};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::programs_bytes::HELLO_WORLD_BYTES;

mod programs_bytes;

#[test]
pub fn hello_world_with_store() {
    let mut bank = LiteSVM::new();

    let payer = Keypair::new();
    let program_bytes = HELLO_WORLD_BYTES;

    bank.airdrop(&payer.pubkey(), 1000000000).unwrap();

    let program_kp = Keypair::new();
    let program_id = program_kp.pubkey();
    bank.store_program(program_id, program_bytes);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        vec![AccountMeta::new(payer.pubkey(), true)],
    );
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx_result = bank.send_message(message, &[&payer]).unwrap();

    assert!(tx_result.result.is_ok());
    assert!(tx_result
        .metadata
        .logs
        .contains(&"Program log: Hello world!".to_string()));
}

#[test]
pub fn hello_world_with_deploy() {
    let mut bank = LiteSVM::new();

    let payer = Keypair::new();
    let program_bytes = HELLO_WORLD_BYTES;

    bank.airdrop(&payer.pubkey(), 1000000000).unwrap();

    let program_id = deploy_program(&mut bank, &payer, program_bytes).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        vec![AccountMeta::new(payer.pubkey(), true)],
    );
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx_result = bank.send_message(message, &[&payer]).unwrap();

    assert!(tx_result.result.is_ok());
    assert!(tx_result
        .metadata
        .logs
        .contains(&"Program log: Hello world!".to_string()));
}

#[test]
pub fn hello_world_with_deploy_upgradeable() {
    let mut bank = LiteSVM::new();

    let payer_kp = Keypair::new();
    let payer_pk = payer_kp.pubkey();
    let program_bytes = HELLO_WORLD_BYTES;

    bank.airdrop(&payer_pk, 10000000000).unwrap();

    let program_id = deploy_upgradeable_program(&mut bank, &payer_kp, program_bytes).unwrap();

    let instruction =
        Instruction::new_with_bytes(program_id, &[], vec![AccountMeta::new(payer_pk, true)]);
    let message = Message::new(&[instruction], Some(&payer_pk));
    let tx_result = bank.send_message(message, &[&payer_kp]).unwrap();

    assert!(tx_result.result.is_ok());
    assert!(tx_result
        .metadata
        .logs
        .contains(&"Program log: Hello world!".to_string()));
}