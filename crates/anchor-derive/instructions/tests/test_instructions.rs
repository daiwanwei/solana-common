use anchor_instructions_derive::Instructions;
use anchor_trait::Discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use solana_program::pubkey::Pubkey;

#[derive(Instructions)]
pub enum Test {
    #[instruction(rename = "woo")]
    TestWoo {
        test: u64,
    },

    TestPig,
}

pub const ID: Pubkey = Pubkey::new_from_array([0; 32]);

#[test]
fn test_discriminator_without_rename() {
    let discriminator = generate_discriminator("global", "test_pig");
    assert_eq!(discriminator, TestPig::DISCRIMINATOR);
    let _test_pig = TestPig {};
}

#[test]
fn test_discriminator_with_rename() {
    let discriminator = generate_discriminator("global", "woo");
    assert_eq!(discriminator, TestWoo::DISCRIMINATOR);
    let _test_woo = TestWoo { test: 1 };
}

fn generate_discriminator(namespace: &str, name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("{namespace}:{name}").as_bytes());
    let discriminator = hasher.finalize();
    discriminator[..8].try_into().unwrap()
}
