use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList, seeds::Seed};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {

    pub fn initialize_transfer_hook(&mut self) -> Result<()>{
        
        let extra = Self::extra_account_metas()?;

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut self.extra_account_meta_list.try_borrow_mut_data()?,
            &extra,
        )
        .unwrap();

        Ok(())
    }

    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        // Derive the whitelist PDA using our program ID

        Ok(vec![ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: b"whitelist".to_vec() },
                Seed::AccountKey { index: 3 }
            ],
            false,
            false,
        )
        .unwrap()])
    }
}
