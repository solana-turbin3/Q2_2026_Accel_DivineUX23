use anchor_lang::{prelude::*};

use crate::state::{Whitelist, WhitelistEntry};


#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddWhitelist<'info> {
    #[account(
        mut,
        //address =
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(
        init,
        payer = admin,
        space = WhitelistEntry::DISCRIMINATOR.len() + WhitelistEntry::INIT_SPACE,
        seeds = [b"whitelist", user.as_ref()],
        bump
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddWhitelist<'info> {
    pub fn add_to_whitelist(&mut self, bumps: &AddWhitelistBumps, user: Pubkey) -> Result<()> {
        self.whitelist_entry.set_inner(WhitelistEntry { 
            address: user, 
            bump: bumps.whitelist_entry, 
        });

        self.whitelist.total_whitelisted += 1;

        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveWhitelist<'info> {
    #[account(
        mut,
        //address =
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", user.as_ref()],
        bump = whitelist_entry.bump
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveWhitelist<'info> {

    pub fn remove_from_whitelist(&mut self) -> Result<()> {

        self.whitelist.total_whitelisted -= 1;

        Ok(())
    }

}