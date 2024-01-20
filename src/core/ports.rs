use crate::core::domain::domain::PresenterCommand;

pub trait PresenterPort {
    fn execute(& mut self, _: PresenterCommand);
}
