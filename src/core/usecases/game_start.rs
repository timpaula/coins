use crate::core::domain::domain::{PresenterCommand, Table};
use crate::core::ports::{PresenterPort, TableStoragePort};

pub(crate) struct StartGameUsecase<'a> {
    pub(crate) presenter: &'a mut dyn PresenterPort,
    pub(crate) table_storage: &'a mut dyn TableStoragePort,
}

impl StartGameUsecase<'_> {
    pub(crate) fn execute(&mut self) {
        self.table_storage.save(Table::default());
        self.presenter.execute(PresenterCommand::StartGame);
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::fake_presenter::FakePresenter;
    use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
    use crate::core::domain::
    domain::PresenterCommand;
    use crate::core::domain::domain::Table;
    use crate::core::ports::TableStoragePort;
    use crate::core::usecases::game_start::StartGameUsecase;

    #[test]
    fn test_game_started() {
        let mut fake_presenter = FakePresenter::default();
        let mut table_storage = InMemoryTableStorage::default();

        // Given
        let mut usecase = StartGameUsecase {
            presenter: &mut fake_presenter,
            table_storage: &mut table_storage
        };

        // When
        usecase.execute();

        // Then
        assert_eq!(PresenterCommand::StartGame, fake_presenter.get_previous_command().unwrap());
        assert_eq!(Table::default(), table_storage.load());
    }

    #[test]
    fn test_game_restarted() {
        todo!()
    }

}