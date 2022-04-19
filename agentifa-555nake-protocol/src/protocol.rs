use naia_shared::Protocolize;

mod auth;
mod head;
mod highscore;
mod highscore_rank;
mod position;

pub use auth::Auth;
pub use head::Head;
pub use highscore::HighScore;
pub use highscore_rank::HighScoreRank;
pub use position::Position;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Head(Head),
    HighScore(HighScore),
    HighScoreRank(HighScoreRank),
    Position(Position),
}
