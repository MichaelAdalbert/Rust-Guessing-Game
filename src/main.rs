use iced::{Application, Settings};
mod guessing_game;

fn main() -> iced::Result {
    guessing_game::game::GuessingGame::run(Settings::default())
}
