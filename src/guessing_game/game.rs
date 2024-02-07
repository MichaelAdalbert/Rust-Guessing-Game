use iced::widget::{button, column, text, text_input};
use iced::{executor, Application, Command, Element, Renderer, Theme};
use rand::random;

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
    fn init(low: u32, high: u32) -> Self {
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

impl Application for GuessingGame {
    type Executor = executor::Default;
    type Message = GuessingGameMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (GuessingGame::init(0, 10), Command::none())
    }

    fn title(&self) -> String {
        format!(
            "Guessing Game {} {}",
            if self.guessing_output.len() == 0 {
                ""
            } else {
                "-"
            },
            self.guessing_output
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GuessingGameMessage::Input(input) => self.guessing_input = input,
            GuessingGameMessage::Guess => {
                self.guessing_output = match self.guess() {
                    Ok(attempts) => {
                        self.party_finished = true;
                        format!("It tooks {attempts} attempts")
                    }
                    Err(err) => format!("{err}"),
                }
            }
            GuessingGameMessage::NewParty => self.reset(),
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let output = text(self.guessing_output.as_str());
        let input = text_input("Enter a number ?", self.guessing_input.as_str())
            .on_input(GuessingGameMessage::Input);

        let input = if self.party_finished {
            input
        } else {
            input.on_submit(GuessingGameMessage::Guess)
        };

        let column = column![output, input];

        if self.party_finished {
            let new_party_button = button("New party ?").on_press(GuessingGameMessage::NewParty);
            column.push(new_party_button)
        } else {
            column
        }
        .into()
    }
}
