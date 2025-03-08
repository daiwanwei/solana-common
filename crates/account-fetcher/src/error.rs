use snafu::Snafu;
use solana_deserialize::account;
use solana_rpc_client_api::client_error::Error as RpcError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AccountError {
    #[snafu(display("Failed to deserialize account: {}", source))]
    Deserialize { source: account::AccountError },

    #[snafu(display("Failed to fetch account: {}", source))]
    FetchAccount { source: RpcError },

    #[snafu(display("Too many pubkeys: expected {}, actual {}", max, actual))]
    TooManyPubkeys { max: usize, actual: usize },
}

pub type Result<T> = std::result::Result<T, AccountError>;
