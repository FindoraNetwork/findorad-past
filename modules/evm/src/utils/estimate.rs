use evm::{ExitReason, ExitSucceed};

pub fn estimate_gas() -> (ExitReason, u64) {
    (ExitReason::Succeed(ExitSucceed::Stopped), 0)
}
