use primitive_types::H160;

#[derive(Debug, Clone)]
pub struct InputOperation {
    pub caller: H160,
}

#[derive(Debug, Clone)]
pub struct OutputOperation {
    pub target: H160,
}
