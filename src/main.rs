use crate::adapters::cli_presenter::CliPresenter;
use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
use crate::core::usecases::game_start::StartGameUsecase;

mod core;
mod adapters;

fn main() {
    let mut presenter = CliPresenter::default();
    let mut table_storage = InMemoryTableStorage::default();

    let mut start_game_usecase = StartGameUsecase {
        presenter: &mut presenter,
        table_storage: &mut table_storage
    };

    start_game_usecase.execute();
}
