use borsh::BorshSerialize;
use solana_program::hash::hashv;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use std::str::FromStr;
use token_registry::entrypoint::process_instruction;
use token_registry::instruction::{init, register};
use token_registry::state::ADMINS;
use token_registry::state::TOKEN_TLD;

use spl_name_service::state::{get_seeds_and_key, HASH_PREFIX};

pub mod common;

use crate::common::utils::sign_send_instructions;

#[tokio::test]
async fn test() {
    // Create program and test environment
    let token_registry_program_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "token_registry",
        token_registry_program_id,
        processor!(process_instruction),
    );

    // Add name service
    program_test.add_program(
        "spl_name_service",
        spl_name_service::id(),
        processor!(spl_name_service::processor::Processor::process_instruction),
    );

    let (state_key, nonce) = Pubkey::find_program_address(
        &[&token_registry_program_id.to_bytes()],
        &token_registry_program_id,
    );

    let root_domain_data = spl_name_service::state::NameRecordHeader {
        parent_name: Pubkey::default(),
        owner: state_key,
        class: Pubkey::default(),
    }
    .try_to_vec()
    .unwrap();

    // Add token TLD
    program_test.add_account(
        Pubkey::from_str(TOKEN_TLD).unwrap(),
        Account {
            lamports: 1_000_000,
            data: root_domain_data,
            owner: spl_name_service::id(),
            ..Account::default()
        },
    );

    // Add admin
    program_test.add_account(
        Pubkey::from_str(ADMINS.get(0).unwrap()).unwrap(),
        Account {
            lamports: 10_000_000,
            ..Account::default()
        },
    );

    // Create test context
    let mut prg_test_ctx = program_test.start_with_context().await;

    // Create central state

    let init_instruction = init(
        token_registry_program_id,
        state_key,
        prg_test_ctx.payer.pubkey(),
        token_registry::instruction::init::Params {
            signer_nonce: nonce,
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![init_instruction], vec![])
        .await
        .unwrap();

    let ticker_name = "FIDA";
    let mint_name = "EchesyfXePKdLtoiZSL8pBe8Myagyy8ZRqsACNCFGnvp";

    let hashed_name_ticker = hashv(&[(HASH_PREFIX.to_owned() + ticker_name).as_bytes()])
        .0
        .to_vec();

    println!("Hashed name length {:?}", hashed_name_ticker.len());

    let (name_ticker_account, _) = get_seeds_and_key(
        &spl_name_service::id(),
        hashed_name_ticker.clone(),
        None,
        Some(&Pubkey::from_str(TOKEN_TLD).unwrap()),
    );

    let hashed_name_mint = hashv(&[(HASH_PREFIX.to_owned() + mint_name).as_bytes()])
        .0
        .to_vec();

    println!("Hashed name length {:?}", hashed_name_mint.len());

    let (name_mint_account, _) = get_seeds_and_key(
        &spl_name_service::id(),
        hashed_name_mint.clone(),
        None,
        Some(&Pubkey::from_str(TOKEN_TLD).unwrap()),
    );

    let register_instruction = register(
        token_registry_program_id,
        prg_test_ctx.payer.pubkey(),
        name_ticker_account,
        name_mint_account,
        Pubkey::from_str(TOKEN_TLD).unwrap(),
        state_key,
        token_registry::instruction::register::Params {
            signer_index: 0,
            name: "Bonfida Token".to_string(),
            ticker: ticker_name.to_string(),
            mint: mint_name.to_string(),
            decimals: 6,
            website: "".to_string(),
            logo_uri: "".to_string(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![register_instruction], vec![])
        .await
        .unwrap();
}
