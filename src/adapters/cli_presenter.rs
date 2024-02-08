use crate::core::domain::domain::{Coin, DomainError, Table};
use crate::core::ports::PresenterPort;

#[derive(Default)]
pub(crate) struct CliPresenter {}

impl PresenterPort for CliPresenter {
    fn show_welcome(&mut self) {
        println!("Game started, welcome!");
    }
    fn show_error(&mut self, err: DomainError) {
        println!("Error: {}", err);
    }
    fn show_table(&mut self, table: &Table) {
        println!("{}", CliPresenter::format_table(table));
    }
}

impl CliPresenter {
    fn format_table(table: &Table) -> String {
        let rows: [[Option<Coin>; 5]; 5] = [0..4]
            .map(|row_index| table.columns().iter()
                    .map(|column| column[row_index])
                    .collect())
            .collect();

        let vector_columns : Vec<Vec<Option<Coin>>> = table.columns();
        let rows: [[Option<Coin>; 5]; 5] = Array2D::from_columns


        String::default()

    }
}

#[cfg(test)]
mod test {
    use crate::adapters::cli_presenter::CliPresenter;
    use crate::core::domain::domain::{Coin, Table};

    #[test]
    fn test_show_table() {
        // Given
        let table = typical_table();

        // When
        let formatted_table = CliPresenter::format_table(&table);

        // Then
        let expected_formatted_table = r#"
        0 0 0 0 0
        0 0 0 R 0
        0 0 R G 0
        0 G G R 0
        G R R G 0
        "#;
        assert_eq!(expected_formatted_table, formatted_table);
    }

    fn typical_table() -> Table {
        [0usize, 1usize, 1usize, 2usize, 2usize, 2usize, 3usize, 3usize, 3usize, 3usize]
            .iter()
            .fold((Table::default(), Coin::Green), |(table, color), &i| {
                (table.with_one_more_coin_at_column(color, i), color.other())
            }).0
    }
}