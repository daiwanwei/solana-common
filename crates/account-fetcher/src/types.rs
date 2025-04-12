use solana_sdk::{pubkey::Pubkey, stake_history::Epoch};

#[derive(Clone, Debug)]
pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: Epoch,
    pub data: T,
}
