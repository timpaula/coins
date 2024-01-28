use crate::core::domain::domain::{PresenterCommand};
use crate::core::ports::{PresenterPort, TableStoragePort};

#[derive(Default)]
struct ThrowCoinUsecase<P: PresenterPort, T: TableStoragePort> {
    presenter: P,
    table_storage: T,
}

impl<P: PresenterPort, T: TableStoragePort> ThrowCoinUsecase<P, T> {
    pub(crate) fn execute(&mut self, num: usize) {
        let mut table = self.table_storage.load();

        let result = table.throw_coin(num);
        if let Err(err) = result {
            self.presenter.execute(PresenterCommand::ShowError(err));
            return;
        }

        let command = PresenterCommand::ShowTable(table.clone());
        self.presenter.execute(command);
        self.table_storage.save(table);
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use log::error;
    use crate::adapters::fake_presenter::FakePresenter;
    use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
    use crate::core::domain::domain::{Coin, DomainError, Table};
    use crate::core::domain::domain::PresenterCommand::{ShowError, ShowTable};
    use crate::core::usecases::throw_coin::ThrowCoinUsecase;

    #[test]
    fn test_throw_in_empty_column() {
        // given
        let mut throw_coin_usecase = ThrowCoinUsecase::<FakePresenter, InMemoryTableStorage>::default();

        // when
        let column_number = 3;
        throw_coin_usecase.execute(column_number);

        // then
        let prev_command = throw_coin_usecase.presenter
            .get_previous_command()
            .expect("Expected something to have been presented.");

        assert!(matches!(prev_command, ShowTable(_)));

        let ShowTable(table) = prev_command else { panic!("Expected table to have been presented.") };

        assert_eq!(Table::default().with_a_coin_at_column(Coin::Green, column_number), table);
    }
    #[test]
    fn test_throw_last_coin_in_column() {
        todo!()
    }
    #[test]
    fn test_throw_in_full_column() {
        todo!()
    }
    #[test]
    fn test_column_number_out_of_range() {
        // given
        let mut throw_coin_usecase = ThrowCoinUsecase::<FakePresenter, InMemoryTableStorage>::default();

        // when
        let column_number = 6;
        throw_coin_usecase.execute(column_number);

        // then
        assert_eq!(ShowError(DomainError::TableColumnOutOfRange(column_number)), throw_coin_usecase.presenter.get_previous_command().unwrap());
    }

}