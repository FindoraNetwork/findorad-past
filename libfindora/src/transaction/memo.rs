#[derive(Debug)]
pub enum Memo {
    Ethereum(ethereum::TransactionV2),
}
