use crate::core::domain::domain::{DomainError, Table};

pub trait PresenterPort {
    fn show_welcome(&mut self);
    fn show_error(&mut self, _: DomainError);
    fn show_table(&mut self, table: &Table);
}

pub trait TableStoragePort {
    fn load(&self) -> Table;
    fn save(&mut self, table: Table);
}
