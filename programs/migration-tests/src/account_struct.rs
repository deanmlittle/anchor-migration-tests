use anchor_lang::prelude::*;

// #[derive(Accounts)]
pub struct MyContext<'info> {
    // #[account(mut)]
    signer: UncheckedAccount<'info>
}

// Recursive expansion of Accounts macro
// ======================================

#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for MyContext<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        __ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs: &mut std::collections::BTreeSet<anchor_lang::solana_program::pubkey::Pubkey>,
    ) -> anchor_lang::Result<Self> {
        #[cfg(feature = "anchor-debug")]
        ::solana_program::log::sol_log(stringify!(signer:UncheckedAccount));
        let signer: UncheckedAccount = anchor_lang::Accounts::try_accounts(
            __program_id,
            __accounts,
            __ix_data,
            __bumps,
            __reallocs,
        )
        .map_err(|e| e.with_account_name("signer"))?;
        if !signer.to_account_info().is_writable {
            return Err(anchor_lang::error::Error::from(
                anchor_lang::error::ErrorCode::ConstraintMut,
            )
            .with_account_name("signer"));
        }
        Ok(MyContext { signer })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for MyContext<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = vec![];
        account_infos.extend(self.signer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for MyContext<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = vec![];
        account_metas.extend(self.signer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for MyContext<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.signer, program_id)
            .map_err(|e| e.with_account_name("signer"))?;
        Ok(())
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a struct for a given"]
#[doc = r" `#[derive(Accounts)]` implementation, where each field is a Pubkey,"]
#[doc = r" instead of an `AccountInfo`. This is useful for clients that want"]
#[doc = r" to generate a list of accounts, without explicitly knowing the"]
#[doc = r" order all the fields should be in."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" `accounts` module (also generated), which re-exports this."]
pub(crate) mod __client_accounts_my_context {
    use super::*;
    use anchor_lang::prelude::borsh;
    #[doc = " Generated client accounts for [`MyContext`]."]
    #[derive(anchor_lang::AnchorSerialize)]
    pub struct MyContext {
        pub signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for MyContext {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.signer,
                false,
            ));
            account_metas
        }
    }
}
#[doc = r" An internal, Anchor generated module. This is used (as an"]
#[doc = r" implementation detail), to generate a CPI struct for a given"]
#[doc = r" `#[derive(Accounts)]` implementation, where each field is an"]
#[doc = r" AccountInfo."]
#[doc = r""]
#[doc = r" To access the struct in this module, one should use the sibling"]
#[doc = r" [`cpi::accounts`] module (also generated), which re-exports this."]
pub(crate) mod __cpi_client_accounts_my_context {
    use super::*;
    #[doc = " Generated CPI struct of the accounts for [`MyContext`]."]
    pub struct MyContext<'info> {
        pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for MyContext<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = vec![];
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.signer),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for MyContext<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = vec![];
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.signer));
            account_infos
        }
    }
}