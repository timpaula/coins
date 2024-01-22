use crate::core::ports::PresenterPort;
use crate::core::domain::domain::PresenterCommand;


#[derive(Default)]
pub(crate) struct FakePresenter {
    previous_command: Option<PresenterCommand>,
}

impl PresenterPort for FakePresenter {
    fn execute(&mut self, presenter_command: PresenterCommand) {
        self.previous_command = Some(presenter_command);
    }
}

impl FakePresenter {
    pub(crate) fn get_previous_command(&self) -> Option<PresenterCommand> {
        self.previous_command
    }
}
