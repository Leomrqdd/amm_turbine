pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C9VNgETz1HMM1G73aVT6b1KVFG2ag1yBQ4PKXuwiCFm5");

#[program]
pub mod amm_turbine {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed:u64,
        fee:u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed,fee,authority,ctx.bumps)
    }

}
