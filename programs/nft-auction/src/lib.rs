use anchor_lang::prelude::*;

declare_id!("HBUhjeux3yF45gErBG2NVNnuoAG9Pck3SVA34ruAcUig");

/*
    TODO:
    1. NFT collection 민트, 토큰 어카운트
    2. NFT 작품 민트, 토큰 어카운트
    3. NFT 작품 Verify
*/

#[program]
pub mod nft_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
