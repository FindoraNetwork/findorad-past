@0xa3264497cdf8c2d7;

using Evm = import "evm.capnp";
using Address = import "address.capnp";
using Governance = import "governance.capnp";

struct ConfidentialAmount {
    point0 @0: Data;
    point1 @1: Data;
}

struct OwnerMemo {
    blindShare @0: Data;
    ctext @1: Data;
    ephemeralPublicKey @2: Data;
}

struct ValidatorKey {
    key :union {
        unknown @0: Void;
        ed25519 @1: Data;
        secp256k1 @2: Data;
    }
}

struct DelegateData {
    address @0: Data;
    validator :union {
        none @1: Void;
        some @2: ValidatorKey;
    }
    memo :union {
        nono @3: Void;
        some @4: Data;
    }
}

struct UndelegateData {
    address @0: Data;
}

struct ClaimData {
    validator @0: Data;
}

struct Output {
    address @0: Address.Address;

    amount :union {
        confidential @1: ConfidentialAmount;
        nonConfidential @2: UInt64;
    }

    asset :union {
        confidential @3: Data;
        nonConfidential @4: Data;
    }

    ownerMemo :union {
        none @5: Void;
        some @6: OwnerMemo;
    }

    operation :union {
        transferAsset @7: Void;
        defineAsset @8: DefineAsset;
        issueAsset @9: Void;
        fee @10: Void;
        undelegate @11: UndelegateData;
        claimReward @12: ClaimData;
        delegate @13: DelegateData;
        evmCall @14: Evm.Output;
        createProposal @15: Governance.CreateProposal;
        voteProposal @16: Governance.VoteProposal;
    }

}

struct DefineAsset {
    transferable @0: Bool;
    maximum :union {
        none @1: Void;
        some @2: Data;
    }
}
