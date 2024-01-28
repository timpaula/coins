use crate::core::domain::domain::{DomainError, Table};
use crate::core::ports::PresenterPort;

#[derive(Default)]
pub(crate) struct CliPresenter {}

impl PresenterPort for CliPresenter {
    fn show_welcome(&mut self) {
        println!("Game started, welcome!");
    }
    fn show_error(&mut self, err: DomainError) {
        println!("Error: {}", err);
    }
    fn show_table(&mut self, table: &Table) {
        todo!()
    }
}