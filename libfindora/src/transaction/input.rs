use primitive_types::H512;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: H512,
    pub n: u32,
    pub operation: Operation,
}
