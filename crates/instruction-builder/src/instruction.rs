use anchor_trait::InstructionData;
use solana_common_core::ToAccountMetas;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(Debug)]
pub struct InstructionBuilder {
    program_id: Pubkey,
    named_accounts: Vec<AccountMeta>,
    remaining_accounts: Vec<AccountMeta>,
    data: Vec<u8>,
}

impl InstructionBuilder {
    pub fn new(program_id: Pubkey) -> Self {
        Self {
            program_id,
            named_accounts: Vec::new(),
            remaining_accounts: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn add_named_accounts_from_struct<T: ToAccountMetas>(&mut self, accounts: T) {
        self.add_named_accounts(accounts.to_account_metas());
    }

    pub fn add_named_accounts(&mut self, accounts: Vec<AccountMeta>) {
        self.named_accounts.extend(accounts);
    }

    pub fn add_remaining_accounts(&mut self, accounts: Vec<AccountMeta>) {
        self.remaining_accounts.extend(accounts);
    }

    pub fn add_data(&mut self, data: &[u8]) { self.data.extend(data); }

    pub fn add_anchor_data<T: InstructionData>(&mut self, data: T) {
        self.data.extend(data.data());
    }

    pub fn build(&self) -> Instruction {
        let total_accounts = self.named_accounts.len() + self.remaining_accounts.len();

        let mut accounts = Vec::with_capacity(total_accounts);
        accounts.extend(self.named_accounts.clone());
        accounts.extend(self.remaining_accounts.clone());

        Instruction { program_id: self.program_id, accounts, data: self.data.clone() }
    }
}
