use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Whitelist::DISCRIMINATOR.len() + Whitelist::INIT_SPACE, // 8 bytes for discriminator, 4 bytes for vector length, 1 byte for bump
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        self.whitelist.set_inner(Whitelist {
            address: self.admin.key(),
            bump: bumps.whitelist,
            total_whitelisted: 0
        });

        Ok(())
    }
}
