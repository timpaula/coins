use crate::core::domain::domain::Table;
use crate::core::ports::{PresenterPort, TableStoragePort};

#[derive(Default)]
pub(crate) struct StartGameUsecase<P: PresenterPort, T: TableStoragePort> {
    pub(crate) presenter: P,
    pub(crate) table_storage: T,
}

impl<P: PresenterPort, T: TableStoragePort> StartGameUsecase<P, T> {
    pub(crate) fn execute(&mut self) {
        self.table_storage.save(Table::default());
        self.presenter.show_welcome();
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::fake_presenter::FakePresenter;
    use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
    use crate::adapters::fake_presenter::PresenterCommand;
    use crate::core::domain::domain::Table;
    use crate::core::ports::TableStoragePort;
    use crate::core::usecases::game_start::StartGameUsecase;

    #[test]
    fn test_game_started() {
        // Given
        let mut usecase = StartGameUsecase::<FakePresenter, InMemoryTableStorage>::default();

        // When
        usecase.execute();

        // Then
        assert_eq!(PresenterCommand::ShowWelcome, usecase.presenter.get_previous_command().unwrap());
        assert_eq!(Table::default(), usecase.table_storage.load());
    }

    #[test]
    fn test_game_restarted() {
        todo!()
    }

}