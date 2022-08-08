use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{
        Token,
        TokenAccount,
        Mint,
    },
    mint,
    associated_token,
    associated_token::AssociatedToken
};
use solana_program::{
    system_program,
    sysvar::rent
};

declare_id!("HBUhjeux3yF45gErBG2NVNnuoAG9Pck3SVA34ruAcUig");

/*
    TODO:
    1. NFT collection 민트, 토큰 어카운트 (v)
    2. NFT 작품 민트, 토큰 어카운트
    3. NFT 작품 Verify
*/

#[program]
pub mod nft_auction {
    use super::*;

    pub fn initialize_creator(ctx: Context<InitializeCreator>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCreator<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        mint::decimals = 0,
        mint::authority = creator
    )]
    pub nft_collection_mint: Account<'info, Mint>,

    #[account(
        associated_token::mint = nft_collection_mint,
        associated_token::authority = creator
    )]
    pub nft_collection_token_account: Account<'info, TokenAccount>,

    /// CHECK: NEED VERIFY
    pub nft_collection_metadata: AccountInfo<'info>,

    /// CHECK: NEED VERIFY
    pub nft_collection_master_edition: AccountInfo<'info>,

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}
