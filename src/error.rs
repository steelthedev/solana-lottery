use anchor_lang::prelude::error_code;

#[error_code]
pub enum LotteryError {
    #[msg("Winner already exist")]
    WinnerAlreadyExists,

    #[msg("Cant choose winner when there are no tickets")]
    NoTickets,

    #[msg("Winner has not been chosen.")]
    WinnerNotChosen,

    #[msg("Invalid winner.")]
    InvalidWinner,

    #[msg("Prize already claimed")]
    AlreadyClaimed,
}
