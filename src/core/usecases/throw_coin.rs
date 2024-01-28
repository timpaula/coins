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
        }

        self.table_storage.save(table);
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::fake_presenter::FakePresenter;
    use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
    use crate::core::domain::domain::DomainError;
    use crate::core::domain::domain::PresenterCommand::ShowError;
    use crate::core::usecases::throw_coin::ThrowCoinUsecase;

    #[test]
    fn test_throw_in_empty_column() {
        todo!()
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