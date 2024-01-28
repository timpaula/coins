use crate::adapters::cli_presenter::CliPresenter;
use crate::adapters::in_memory_table_storage::InMemoryTableStorage;
use crate::core::usecases::game_start::StartGameUsecase;

mod core;
mod adapters;

fn main() {
    let mut start_game_usecase = StartGameUsecase::<CliPresenter, InMemoryTableStorage>::default();

    start_game_usecase.execute();
}
