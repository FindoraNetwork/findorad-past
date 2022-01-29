use abcf::tm_protos::abci::RequestBeginBlock;
use libfindora::staking::TendermintAddress;

#[derive(Debug, Clone)]
pub enum ByzantineKind {
    DuplicateVote,
    LightClientAttack,
    OffLine,
    Unknown,
}

impl ByzantineKind {
    pub fn penalty_rate(&self) -> [u64; 2] {
        match self {
            ByzantineKind::DuplicateVote => [5, 100],
            ByzantineKind::LightClientAttack => [1, 100],
            ByzantineKind::OffLine => [1, 1000_0000],
            ByzantineKind::Unknown => [30, 100],
        }
    }
    pub fn from_evidence_type(ty: i32) -> Self {
        match ty {
            0 => Self::Unknown,
            1 => Self::DuplicateVote,
            2 => Self::LightClientAttack,
            _ => {
                // Panic here, beacuse this error is caused by tendermint.
                panic!("Receive error evidence index number from tendermint.");
            }
        }
    }
}

pub struct Evidence {
    pub kind: ByzantineKind,
    pub validator: Option<TendermintAddress>,
}

pub struct BlockEvidence {
    pub evidences: Vec<Evidence>,
}

impl From<&RequestBeginBlock> for BlockEvidence {
    fn from(req: &RequestBeginBlock) -> Self {
        let mut evidences = Vec::new();

        for ev in &req.byzantine_validators {
            let kind = ByzantineKind::from_evidence_type(ev.r#type);
            let validator = ev
                .validator
                .as_ref()
                .map(|validator| TendermintAddress::from(&validator.address));

            evidences.push(Evidence { kind, validator });
        }

        Self { evidences }
    }
}
