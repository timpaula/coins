use crate::core::domain::domain::PresenterCommand;
use crate::core::ports::PresenterPort;

#[derive(Default)]
pub(crate) struct CliPresenter {}

impl PresenterPort for CliPresenter {
    fn execute(&mut self, command: PresenterCommand) {
        println!("{:?}", command);
    }
}