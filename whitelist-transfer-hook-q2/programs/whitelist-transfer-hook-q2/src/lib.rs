#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

use anchor_lang::prelude::*;
use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

declare_id!("AFKCAezosrLJfAF88rnWV3Aic6JZ2HwbD4XA3WhiRE6y");

#[program]
pub mod whitelist_transfer_hook_q2 {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
        ctx.accounts.initialize_whitelist(ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<AddWhitelist>, user: Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(&ctx.bumps, user)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveWhitelist>, user: Pubkey) -> Result<()> {
        ctx.accounts.remove_from_whitelist()
    }

    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        ctx.accounts.initialize_transfer_hook()
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
