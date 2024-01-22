use std::fmt::{Debug};
use thiserror::Error;
use crate::core::domain::domain::DomainError::{ColumnFull, TableColumnOutOfRange};

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum PresenterCommand {
    StartGame,
    ShowError(DomainError),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Coin {
    Green,
    Red,
}

impl Coin {
    pub fn other(&self) -> Coin {
        match self {
            Coin::Green => Coin::Red,
            Coin::Red => Coin::Green,
        }
    }
}

/// In each column, the first index (0) refers to the bottom of the column. And increases upwards.
#[derive(Default, Clone, Debug, PartialEq)]
struct TableColumn([Option<Coin>; 5]);

impl TableColumn {
    pub(crate) fn available_spot_count(&self) -> usize {
        self.0.iter().filter(|&spot|spot.is_none()).count()
    }
    pub(crate) fn get_first_available_index(&self) -> Option<usize> {
        self.0.iter().position(|spot| spot.is_none())
    }

    pub(crate) fn put_coin(&mut self, index: usize, coin: Coin) {
        self.0[index] = Some(coin);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Table {
    columns: [TableColumn; 5],
    next_playing_color: Coin,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            next_playing_color: Coin::Green,
        }
    }
}

impl Table {
   pub fn throw_coin(&mut self, column_index: usize) -> Result<(), DomainError> {
       if column_index >= self.columns.len() {
           return Err(TableColumnOutOfRange(column_index));
       }

       let column = &mut self.columns[column_index];
       let index_of_first_available_spot = column.get_first_available_index().ok_or(ColumnFull(column_index))?;

       let next_playing_color = self.next_playing_color;
       column.put_coin(index_of_first_available_spot, next_playing_color);


       self.next_playing_color = self.next_playing_color.other();

       Ok(())
    }

    pub fn next_playing_color(&self) -> Coin {
        self.next_playing_color
    }
}

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub(crate) enum DomainError {
    #[error("Column index '{0}' out of range")]
    TableColumnOutOfRange(usize),
    #[error("Column '{0}' already full")]
    ColumnFull(usize),
}

#[cfg(test)]
mod test {
    use crate::core::domain::domain::DomainError::{TableColumnOutOfRange, ColumnFull};
    use crate::core::domain::domain::{Coin, Table};

    #[test]
    fn test_throw_coin_column_index_out_of_range() {
        let mut table = Table::default();
        let result = table.throw_coin(6);
        assert_eq!(TableColumnOutOfRange(6), result.unwrap_err());
    }

    #[test]
    fn test_throw_coin() {
        let mut table = Table::default();

        let coin = table.next_playing_color();
        assert_eq!(Coin::Green, coin, "expected first coin on a default table to be green");

        let result = table.throw_coin(4);
        assert!(result.is_ok());

        let coin = table.next_playing_color();
        assert_eq!(Coin::Red, coin, "expected next coin after first throw to be red");

        let column = &table.columns[4];
        assert_eq!(4, column.available_spot_count(), "Expected specific number of available spots.");
    }


    #[test]
    fn test_throw_coin_in_full_column() {
        // Given
        let mut table = Table::default();
        for _ in 0..5 {
            let result = table.throw_coin(0);
            assert!(result.is_ok());
        }

        // When
        let result = table.throw_coin(0);

        // Then
        assert_eq!(ColumnFull(0), result.unwrap_err(), "expected column to be full");
    }

    #[test]
    fn test_throw_multiple_coins() {
        let mut table = Table::default();
        [Coin::Green, Coin::Red, Coin::Green, Coin::Red].iter().enumerate().for_each(|(index, &expected_coin)|{
            let coin = table.next_playing_color();
            assert_eq!(expected_coin, coin, "expected after {:?}th throw the next coin to be {:?}", index, expected_coin);

            let result = table.throw_coin(index); // throwing one to each column
            assert!(result.is_ok());
        });
    }

}