use anchor_lang::prelude::*;

use crate::UserAccountState;

#[derive(Accounts)]
pub struct InitializeUserConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + UserAccountState::INIT_SPACE,
        seeds = [b"user" , payer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUserConfig<'info> {
    pub fn init_user(&mut self,bumps:&InitializeUserConfigBumps) -> Result<()> {
        self.user_account.set_inner(UserAccountState {
            bump:bumps.user_account,
            amount_staked: 0,
            points: 0,
        });
        Ok(())
    }
}
