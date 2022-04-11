use naia_derive::ProtocolType;

mod auth;
mod highscore;
mod highscore_rank;

pub use auth::Auth;
pub use highscore::HighScore;
pub use highscore_rank::HighScoreRank;

#[derive(ProtocolType)]
pub enum Protocol {
    Auth(Auth),
    HighScore(HighScore),
    HighScoreRank(HighScoreRank),
}
