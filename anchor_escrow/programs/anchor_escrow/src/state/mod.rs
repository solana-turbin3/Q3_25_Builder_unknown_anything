use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed:u64,
    pub receive:u64,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub bump:u8,
    pub maker:Pubkey
}