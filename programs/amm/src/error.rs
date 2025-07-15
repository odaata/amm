use anchor_lang::error_code;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Invalid fee amount.")]
    InvalidAmount,
    #[msg("Invalid amount.")]
    InvalidFeeAmount,
    #[msg("Invalid precision.")]
    InvalidPrecision,
    #[msg("Overflow.")]
    Overflow,
    #[msg("This pool is locked.")]
    PoolLocked,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
    #[msg("Udnerflow.")]
    Underflow,
    #[msg("Zero balance.")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::DefaultError,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFeeAmount,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
        }
    }
}
