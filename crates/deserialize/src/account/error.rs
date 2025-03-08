use std::io;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AccountError {
    #[snafu(display("Invalid discriminator length: expected 8, actual {}", actual))]
    InvalidDiscriminatorLength { actual: usize },

    #[snafu(display("Failed to parse discriminator"))]
    ParseDiscriminator,

    #[snafu(display("Invalid discriminator: expected {:?}, actual {:?}", expected, actual))]
    InvalidDiscriminator { expected: [u8; 8], actual: [u8; 8] },

    #[snafu(display("Failed to deserialize account"))]
    DeserializeAnchorAccount { source: io::Error },

    #[snafu(display("Failed to deserialize account"))]
    DeserializeSolanaAccount,
}

pub type Result<T> = std::result::Result<T, AccountError>;
