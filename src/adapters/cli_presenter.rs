use crate::core::domain::domain::PresenterCommand;
use crate::core::ports::PresenterPort;

#[derive(Default)]
pub(crate) struct CliPresenter {}

impl PresenterPort for CliPresenter {
    fn execute(&mut self, command: PresenterCommand) {
        let display_text = match command {
            PresenterCommand::StartGame => {
                String::from("Game started, welcome!")
            },
            PresenterCommand::ShowError(error) => {
                format!("Error: {}", error)
            }
        };
        println!("{}", display_text);
    }
}