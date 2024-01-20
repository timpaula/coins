use std::fmt::{Display};

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum PresenterCommand {
    StartGame,
}
#[derive(Default, Clone, Debug, PartialEq)]
enum CoinSpot {
    #[default]
    Empty,
    Red,
    Green
}
#[derive(Default, Clone, Debug, PartialEq)]
struct TableColumn([CoinSpot; 5]);

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) struct Table {
    columns: [TableColumn; 5],
}

