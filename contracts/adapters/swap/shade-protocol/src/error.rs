use cosmwasm_std::{OverflowError, StdError};
use cw_utils;
use secret_skip::error::SkipError;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Overflow(#[from] OverflowError),

    #[error(transparent)]
    Skip(#[from] SkipError),

    #[error(transparent)]
    Payment(#[from] cw_utils::PaymentError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("swap_operations cannot be empty")]
    SwapOperationsEmpty,

    #[error("coin_in denom must match the first swap operation's denom in")]
    CoinInDenomMismatch,

    #[error("coin_out denom must match the last swap operation's denom out")]
    CoinOutDenomMismatch,

    #[error("Operation exceeds max spread limit")]
    MaxSpreadAssertion,

    #[error("Contract has no balance of offer asset")]
    NoOfferAssetAmount,

    #[error("Snip20 Coin Sent To Contract Does Not Match Asset")]
    InvalidSnip20Coin,
}
