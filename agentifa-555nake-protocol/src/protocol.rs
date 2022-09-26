use naia_shared::Protocolize;

mod assign_msg;
mod auth;
mod dir_cmd;
mod food;
mod head;
mod highscore;
mod highscore_rank;
mod position;
mod quit_cmd;
mod score;
mod segment;
mod start_cmd;
mod vincible;

pub use assign_msg::AssignMsg;
pub use auth::Auth;
pub use dir_cmd::DirCmd;
pub use food::Food;
pub use head::{Direction, Head};
pub use highscore::HighScore;
pub use highscore_rank::HighScoreRank;
pub use position::Position;
pub use quit_cmd::QuitCmd;
pub use score::Score;
pub use segment::Segment;
pub use start_cmd::StartCmd;
pub use vincible::Vincible;

pub const GRID_SIZE: usize = 10;

#[derive(Protocolize)]
pub enum Protocol {
    AssignMsg(AssignMsg),
    Auth(Auth),
    DirCmd(DirCmd),
    Food(Food),
    Head(Head),
    HighScore(HighScore),
    HighScoreRank(HighScoreRank),
    Position(Position),
    QuitCmd(QuitCmd),
    Score(Score),
    Segment(Segment),
    StartCmd(StartCmd),
    Vincible(Vincible),
}
