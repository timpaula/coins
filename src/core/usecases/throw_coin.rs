use crate::core::ports::PresenterPort;

struct ThrowCoinUsecase<'a> {
    presenter: &'a dyn PresenterPort,
}

impl ThrowCoinUsecase<'_> {
    fn execute(num: usize) {
        todo!()
    }
}

#[cfg(test)]
mod test {
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
        todo!()
    }

}