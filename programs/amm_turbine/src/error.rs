use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError{
    #[msg("The pool is currently locked")]
    PoolLocked,
    #[msg("Invalid amount, must be greater than 0")]
    InvalidAmount,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
}
