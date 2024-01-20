use crate::core::domain::domain::Table;
use crate::core::ports::TableStoragePort;

#[derive(Default)]
pub(crate) struct InMemoryTableStorage {
    table: Table,
}

impl TableStoragePort for InMemoryTableStorage {
    fn load(&self) -> Table {
        self.table.clone()
    }

    fn save(&mut self, table: Table) {
        self.table = table
    }
}