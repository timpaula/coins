use crate::core::domain::domain::{PresenterCommand};
use crate::core::ports::{PresenterPort, TableStoragePort};

struct ThrowCoinUsecase<'a> {
    presenter: &'a mut dyn PresenterPort,
    table_storage: &'a mut dyn TableStoragePort,
}

impl ThrowCoinUsecase<'_> {
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
        let mut fake_presenter = FakePresenter::default();
        let mut storage = InMemoryTableStorage::default();

        let mut throw_coin_usecase = ThrowCoinUsecase { presenter: &mut fake_presenter, table_storage: &mut storage };

        // when
        let column_number = 6;
        throw_coin_usecase.execute(column_number);

        // then
        assert_eq!(ShowError(DomainError::TableColumnOutOfRange(column_number)), fake_presenter.get_previous_command().unwrap());
    }

}