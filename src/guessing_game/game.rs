use iced::widget::{button, column, text, text_input};
use iced::{Element, Task as Command};
use rand::random;
use std::ops::Not;

use super::error::GuessingGameError;
use super::message::GuessingGameMessage;

pub struct GuessingGame {
    guessing_input: String,
    guessing_output: String,

    attempt: u32,
    hidden_number: u32,
    low: u32,
    high: u32,
    party_finished: bool,
}

impl GuessingGame {
    pub fn init(low: u32, high: u32) -> Self {
        Self {
            guessing_input: String::default(),
            guessing_output: String::default(),
            attempt: 0,
            hidden_number: (random::<u32>() % (high - low)) + low,
            low,
            high,
            party_finished: false,
        }
    }

    fn reset(&mut self) {
        self.guessing_input.clear();
        self.guessing_output.clear();
        self.party_finished = false;
        self.attempt = 0;
        self.hidden_number = (random::<u32>() % (self.high - self.low)) + self.low;
    }

    fn verify(&self, guess: u32) -> Result<u32, GuessingGameError> {
        if guess == self.hidden_number {
            Ok(self.attempt)
        } else if guess < self.hidden_number && guess >= self.low {
            Err(GuessingGameError::IsLower)
        } else if guess > self.hidden_number && guess <= self.high {
            Err(GuessingGameError::IsHigher)
        } else {
            Err(GuessingGameError::IsNotInRange)
        }
    }

    fn guess(&mut self) -> Result<u32, GuessingGameError> {
        self.attempt += 1;
        self.verify(self.guessing_input.trim().parse()?)
    }
}

impl GuessingGame {
    pub(crate) fn new() -> (Self, Command<GuessingGameMessage>) {
        (GuessingGame::init(0, 10), Command::none())
    }

    pub(crate) fn title(&self) -> String {
        format!(
            "Guessing Game {} {}",
            if self.guessing_output.is_empty() {
                ""
            } else {
                "-"
            },
            self.guessing_output
        )
    }

    pub(crate) fn update(&mut self, message: GuessingGameMessage) -> Command<GuessingGameMessage> {
        match message {
            GuessingGameMessage::Input(input) => self.guessing_input = input,
            GuessingGameMessage::Guess => {
                let party_result = self.guess();
                self.party_finished = party_result.is_ok();
                self.guessing_output = match party_result {
                    Ok(attempts) => format!("It took {attempts} attempts"),
                    Err(err) => format!("{err}"),
                }
            }
            GuessingGameMessage::NewParty => self.reset(),
        };
        Command::none()
    }

    pub(crate) fn view(&self) -> Element<GuessingGameMessage> {
        let output = text(&self.guessing_output);
        let input = text_input("Enter a number ?", &self.guessing_input)
            .on_input(GuessingGameMessage::Input)
            .on_submit_maybe(
                self.party_finished
                    .not()
                    .then(|| GuessingGameMessage::Guess),
            );

        column![output, input]
            .push_maybe(
                self.party_finished
                    .then(|| button("New Party").on_press(GuessingGameMessage::NewParty)),
            )
            .into()
    }
}
