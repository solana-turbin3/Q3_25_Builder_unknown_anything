use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{MasterEditionAccount, Metadata, MetadataAccount}, token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked}
};

use crate::{error::MarketplaceError, Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"marketplace",marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + Listing::INIT_SPACE,
        seeds = [b"listing",maker_mint.key().as_ref(),marketplace.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [b"metadata",maker_mint.key().as_ref(),metadata_program.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref() @MarketplaceError::InvalidCollection,
        constraint = metadata.collection.as_ref().unwrap().verified == true @MarketplaceError::UnverifedCollection
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), maker_mint.key().as_ref(),b"edition"],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> List<'info> {
    pub fn create_listing(&mut self,price:u64,bumps:&ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker:self.maker.key(),
            price,
            nft_mint: self.maker_mint.key(),
            bump: bumps.listing,
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from:self.maker_ata.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }
}
