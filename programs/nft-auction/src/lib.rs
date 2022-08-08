use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{
        Token,
        TokenAccount,
        Mint,
        MintTo,
        mint_to
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
        ctx.accounts.mint_collection_token()?;
        ctx.accounts.create_master_edition()?;
        Ok(())
    }

    pub fn create_nft(ctx: Context<CreateNft>) -> Result<()> {
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
        mint::authority = creator,
        seeds = [
            creator.key().as_ref(),
            b"collection".as_ref()
        ],
        bump
    )]
    pub nft_collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_collection_mint,
        associated_token::authority = creator
    )]
    pub nft_collection_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            nft_collection_mint.key().as_ref()
        ],
        bump = 254,
        seeds::program = mpl_token_metadata::ID
    )]
    pub nft_collection_metadata: AccountInfo<'info>,

    /// CHECK: NFT 콜렉션 마스터 에디션 PDA
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            nft_collection_mint.key().as_ref(),
            b"edition".as_ref()
        ],
        bump = 254,
        seeds::program = mpl_token_metadata::ID
    )]
    pub nft_collection_master_edition: AccountInfo<'info>,

    /// CHECK: 메타데이터 프로그램
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: AccountInfo<'info>,
    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}

impl<'info> InitializeCreator<'info> {
    pub fn mint_collection_token(&self) -> Result<()> {
        mint_to(
            CpiContext::new(
                self.metadata_program.to_account_info(),
                MintTo {
                    mint: self.nft_collection_mint.to_account_info(),
                    to: self.nft_collection_token_account.to_account_info(),
                    authority: self.creator.to_account_info()
                }
            ),
            1 as u64
        )
    }
    pub fn create_master_edition(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}