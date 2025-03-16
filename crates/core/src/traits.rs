use solana_sdk::instruction::AccountMeta;

pub trait ToAccountMetas {
    fn to_account_metas(&self) -> Vec<AccountMeta>;
}
