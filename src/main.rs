use crate::adapters::cli_presenter::CliPresenter;
use crate::core::usecases::game_start::StartGameUsecase;

mod core;
mod adapters;

fn main() {
    let mut presenter = CliPresenter::default();

    let mut start_game_usecase = StartGameUsecase{ presenter: &mut presenter };
    start_game_usecase.execute();
}
