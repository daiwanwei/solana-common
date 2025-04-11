use anchor_trait::Discriminator;
use borsh::BorshDeserialize;
use snafu::ResultExt;
use solana_account::Account;
use solana_clock::Slot;
use solana_deserialize::account::{deserialize_anchor_account, deserialize_solana_account};
use solana_program_pack::Pack;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    constants::MAX_FETCH_ACCOUNTS,
    error::{self, Result},
    types::DecodedAccount,
};

pub async fn fetch_anchor_account<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
) -> Result<(Option<DecodedAccount<T>>, Slot)>
where
    T: BorshDeserialize + Discriminator,
{
    fetch_and_deserialize_account(client, pubkey, commitment_config, |data| {
        deserialize_anchor_account(data).ok()
    })
    .await
}

pub async fn fetch_anchor_accounts<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(Option<DecodedAccount<T>>, Slot)>>
where
    T: BorshDeserialize + Discriminator,
{
    fetch_and_deserialize_accounts(client, pubkeys, commitment_config, |data| {
        deserialize_anchor_account(data).ok()
    })
    .await
}

pub async fn fetch_solana_account<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
) -> Result<(Option<DecodedAccount<T>>, Slot)>
where
    T: Pack,
{
    fetch_and_deserialize_account(client, pubkey, commitment_config, |data| {
        deserialize_solana_account(data).ok()
    })
    .await
}

pub async fn fetch_solana_accounts<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(Option<DecodedAccount<T>>, Slot)>>
where
    T: Pack,
{
    fetch_and_deserialize_accounts(client, pubkeys, commitment_config, |data| {
        deserialize_solana_account(data).ok()
    })
    .await
}

pub async fn fetch_accounts(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(Option<Account>, Slot)>> {
    if pubkeys.is_empty() {
        return Ok(Vec::new());
    }

    let mut accounts = Vec::with_capacity(pubkeys.len());

    for chunk in pubkeys.chunks(MAX_FETCH_ACCOUNTS) {
        let chunk_accounts = client
            .get_multiple_accounts_with_commitment(chunk, commitment_config)
            .await
            .context(error::FetchAccountSnafu)?;

        let slot = chunk_accounts.context.slot;

        let chunk_accounts =
            chunk_accounts.value.into_iter().map(|account| (account, slot)).collect::<Vec<_>>();
        accounts.extend(chunk_accounts);
    }

    Ok(accounts)
}

pub async fn fetch_and_deserialize_account<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
    deserialize: impl Fn(&[u8]) -> Option<T>,
) -> Result<(Option<DecodedAccount<T>>, Slot)> {
    let res = client
        .get_account_with_commitment(&pubkey, commitment_config)
        .await
        .context(error::FetchAccountSnafu)?;

    let slot = res.context.slot;
    let account = res.value.and_then(|account| {
        deserialize(&account.data).map(|data| DecodedAccount {
            lamports: account.lamports,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data,
        })
    });
    Ok((account, slot))
}

pub async fn fetch_and_deserialize_accounts<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
    deserialize: impl Fn(&[u8]) -> Option<T>,
) -> Result<Vec<(Option<DecodedAccount<T>>, Slot)>> {
    if pubkeys.is_empty() {
        return Ok(vec![]);
    }

    let accounts = fetch_accounts(client, pubkeys, commitment_config).await?;

    let accounts = accounts
        .into_iter()
        .map(|(account, slot)| {
            (
                account.and_then(|acc| {
                    deserialize(&acc.data).map(|data| DecodedAccount {
                        lamports: acc.lamports,
                        owner: acc.owner,
                        executable: acc.executable,
                        rent_epoch: acc.rent_epoch,
                        data,
                    })
                }),
                slot,
            )
        })
        .collect::<Vec<_>>();

    Ok(accounts)
}
