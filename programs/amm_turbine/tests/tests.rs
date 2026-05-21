use {
    anchor_spl::associated_token,
    litesvm::LiteSVM,
    litesvm_token::CreateMint,
    solana_message::{Instruction,Message, VersionedMessage},
    solana_signer::Signer,
    solana_keypair::Keypair,
    solana_transaction::versioned::VersionedTransaction,
    solana_pubkey::Pubkey,
    amm_turbine::state::Config,
    anchor_lang::AccountDeserialize,
};

mod ix_handlers;
use ix_handlers::*;



fn send(
    svm: &mut LiteSVM,
    ixs:&[Instruction],
    payer: &Keypair,
    signers: &[&dyn Signer]
) -> litesvm::types::TransactionResult {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap();
    svm.send_transaction(tx)
}


fn setup() -> (
    LiteSVM,
    Keypair,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,

) {
    let program_id = amm_turbine::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/amm_turbine.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let mint_x = CreateMint::new(&mut svm, &payer)
    .decimals(6)
    .authority(&payer.pubkey())
    .send()
    .unwrap();


    let mint_y = CreateMint::new(&mut svm, &payer)
    .decimals(6)
    .authority(&payer.pubkey())
    .send()
    .unwrap();

    let config = Pubkey::find_program_address(
        &[b"config", 0u64.to_le_bytes().as_ref()],
        &program_id,
    ).0;

    let mint_lp = Pubkey::find_program_address(
        &[b"lp",config.as_ref()],
        &program_id,
    ).0;

    let vault_x = associated_token::get_associated_token_address(
        &config,
        &mint_x,
    );

    let vault_y = associated_token::get_associated_token_address(
        &config,
        &mint_y,
    );      


    (
        svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    )



}

#[test]
fn test_initialize() {
    let (
        mut svm,
        payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y,
    ) = setup();

    let ix = create_initialize_ix(
        &payer,
        mint_x,
        mint_y,
        config,
        mint_lp,
        vault_x,
        vault_y
    );

    let res = send(&mut svm, &[ix], &payer, &[&payer]);
    assert!(res.is_ok());

    let account = svm.get_account(&config).unwrap();
    let config_state = Config::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(config_state.fee, 30);
    assert_eq!(config_state.seed, 0);
    assert_eq!(config_state.mint_x, mint_x);
    assert_eq!(config_state.mint_y, mint_y);
    assert_eq!(config_state.locked, false);
    assert_eq!(config_state.authority, Some(payer.pubkey()));


}