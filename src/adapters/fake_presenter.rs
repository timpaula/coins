use crate::adapters::fake_presenter::PresenterCommand::{ShowError, ShowTable, ShowWelcome};
use crate::core::ports::PresenterPort;
use crate::core::domain::domain::{DomainError, Table};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum PresenterCommand {
    ShowWelcome,
    ShowTable(Table),
    ShowError(DomainError),
}

#[derive(Default)]
pub(crate) struct FakePresenter {
    previous_command: Option<PresenterCommand>,
}

impl PresenterPort for FakePresenter {
    fn show_error(&mut self, err: DomainError) {
        self.previous_command = Some(ShowError(err));
    }
    fn show_welcome(&mut self) {
        self.previous_command = Some(ShowWelcome);
    }
    fn show_table(&mut self, table: &Table) {
        self.previous_command = Some(ShowTable(table.clone()));
    }
}

impl FakePresenter {
    pub(crate) fn get_previous_command(&self) -> Option<PresenterCommand> {
        self.previous_command.clone()
    }
}

