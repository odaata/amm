use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

impl Space for Config {
    const INIT_SPACE: usize = 1*2 // bumps
        + 1 // locked
        + 2 // fee
        + 32*2 // mint keys
        + 32 // authority key
        + 1 // Option bit for authority key
        + 8; // seed
}
