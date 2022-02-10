use evm::{ExitReason, ExitSucceed};

pub fn run() -> (ExitReason, Vec<u8>) {
    (ExitReason::Succeed(ExitSucceed::Stopped), (Vec::new()))
}
