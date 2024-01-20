use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum PresenterCommand {
    StartGame,
}

