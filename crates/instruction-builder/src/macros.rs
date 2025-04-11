#[macro_export]
macro_rules! prepare_anchor_ix {
    ($program_id:expr, $ix:expr, $accounts:expr) => {{
        let mut builder = $crate::InstructionBuilder::new($program_id)
            .add_anchor_data($ix)
            .add_named_accounts_from_struct($accounts);

        builder.build()
    }};

    ($program_id:expr, $ix:expr, $accounts:expr, $remaining_accounts:expr) => {{
        let mut builder = $crate::InstructionBuilder::new($program_id)
            .add_anchor_data($ix)
            .add_named_accounts_from_struct($accounts)
            .add_remaining_accounts($remaining_accounts);

        builder.build()
    }};
}
