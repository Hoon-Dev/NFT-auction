use anchor_lang::prelude::*;

declare_id!("HBUhjeux3yF45gErBG2NVNnuoAG9Pck3SVA34ruAcUig");

#[program]
pub mod nft_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
