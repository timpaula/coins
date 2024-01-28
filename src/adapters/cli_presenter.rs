use crate::core::domain::domain::PresenterCommand;
use crate::core::ports::PresenterPort;

#[derive(Default)]
pub(crate) struct CliPresenter {}

impl PresenterPort for CliPresenter {
    fn execute(&mut self, command: PresenterCommand) {
        let display_text = match command {
            PresenterCommand::ShowWelcome => {
                String::from("Game started, welcome!")
            },
            PresenterCommand::ShowError(error) => {
                format!("Error: {}", error)
            },
            PresenterCommand::ShowTable(_) => {todo!()},
        };
        println!("{}", display_text);
    }
}