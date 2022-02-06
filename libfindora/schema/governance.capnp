@0xe206e3616a717781;

using Input = import "input.capnp";

struct UpdateRewardsRuleProposal {
    code @0: Data;
}

struct Proposal {
    updateRewards @0: UpdateRewardsRuleProposal;
}

struct CreateProposal {
    height @0 :Int64;
    proposal @1 :Proposal;
}

struct VoteProposal {
    id @0 :Input.Input;
}

