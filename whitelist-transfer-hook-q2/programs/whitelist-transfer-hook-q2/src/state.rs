use anchor_lang::prelude::*;
/* 
#[account]
pub struct Whitelist {
    pub address: Vec<Pubkey>,
    pub bump: u8,
}
*/
#[derive(InitSpace)]
#[account]
pub struct Whitelist {
    pub address: Pubkey,
    pub bump: u8,
    pub total_whitelisted: u8
}

#[derive(InitSpace)]
#[account]
pub struct WhitelistEntry {
    pub address: Pubkey,
    pub bump: u8,
}