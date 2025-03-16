use solana_accounts_derive::ToAccountMetas;
use solana_common_core::ToAccountMetas;
use solana_sdk::pubkey::Pubkey;

#[derive(ToAccountMetas)]
pub struct TransferAccounts {
    #[account(mut, is_signer = true)]
    pub authority: Pubkey,
    #[account(mut)]
    pub from: Option<Pubkey>,
    #[account(is_signer = false)]
    pub to: Pubkey,
}

pub const ID: Pubkey = Pubkey::new_from_array([0; 32]);

#[test]
fn test_transfer_accounts() {
    let accounts =
        TransferAccounts { authority: Pubkey::new_unique(), from: None, to: Pubkey::new_unique() };

    let meta_accounts = accounts.to_account_metas();
    assert_eq!(meta_accounts.len(), 3);

    println!("{:?}", meta_accounts);

    assert!(meta_accounts[0].is_signer);
    assert!(meta_accounts[0].is_writable);
    assert_eq!(meta_accounts[0].pubkey, accounts.authority);

    assert!(!meta_accounts[1].is_signer);
    assert!(meta_accounts[1].is_writable);
    assert_eq!(meta_accounts[1].pubkey, accounts.from.unwrap_or(ID));

    assert!(!meta_accounts[2].is_signer);
    assert!(!meta_accounts[2].is_writable);
    assert_eq!(meta_accounts[2].pubkey, accounts.to);
}
