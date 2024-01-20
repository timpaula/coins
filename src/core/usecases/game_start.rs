use crate::core::domain::domain::PresenterCommand;
use crate::core::ports::PresenterPort;

pub(crate) struct StartGameUsecase<'a> {
    pub(crate) presenter: &'a mut dyn PresenterPort,
}

impl StartGameUsecase<'_> {
    pub(crate) fn execute(&mut self) {
        self.presenter.execute(PresenterCommand::StartGame);
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::fake_presenter::FakePresenter;
    use crate::core::domain::
    domain::PresenterCommand;
    use crate::core::usecases::game_start::StartGameUsecase;

    #[test]
    fn test_clear_table() {
        let mut fake_presenter = FakePresenter::default();

        // Given
        let mut usecase = StartGameUsecase{ presenter: &mut fake_presenter };
        // When
        usecase.execute();
        // Then
        assert_eq!(PresenterCommand::StartGame, fake_presenter.get_previous_command().unwrap());

    }
}