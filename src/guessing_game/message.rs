#[derive(Debug, Clone)]
pub enum GuessingGameMessage {
    Input(String),
    Guess,
    NewParty,
}
