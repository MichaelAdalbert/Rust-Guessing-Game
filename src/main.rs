use crate::guessing_game::game::GuessingGame;

mod guessing_game;

fn main() -> iced::Result {
    iced::application(GuessingGame::title, GuessingGame::update, GuessingGame::view)
        .window_size((500.0, 800.0))
        .run_with(GuessingGame::new)
}
