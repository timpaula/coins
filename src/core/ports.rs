use crate::core::domain::domain::{DomainError, PresenterCommand, Table};

pub trait PresenterPort {
    fn execute(& mut self, _: PresenterCommand);

}

pub trait TableStoragePort {
    fn load(&self) -> Table;
    fn save(&mut self, table: Table);
}
