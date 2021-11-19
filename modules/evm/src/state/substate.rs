use std::{collections::BTreeSet, mem};

use ethereum::Log;
use evm::{executor::stack::StackSubstateMetadata, Transfer};
use primitive_types::H160;

pub enum RunningState {
    Waiting,
    Running,
    Success,
    Revert,
}

impl Default for RunningState {
    fn default() -> Self {
        Self::Waiting
    }
}

pub struct Substate<'config> {
    pub metadata: StackSubstateMetadata<'config>,
    pub logs: Vec<Log>,
    pub running_state: RunningState,
    pub deletes: BTreeSet<H160>,
}

impl<'config> Substate<'config> {
    pub fn log(&mut self, log: Log) {
        self.logs.push(log)
    }

    pub fn start(&mut self, gas_limit: u64, is_static: bool) {
        let mut entering = Self {
            metadata: self.metadata.spit_child(gas_limit, is_static),
            logs: Vec::new(),
            deletes: BTreeSet::new(),
            running_state: RunningState::Running,
        };

        self.running_state = RunningState::Running;

        mem::swap(&mut entering, self);
    }
}
