@0x9edb1e9495ce4d5a;

using Evm = import "evm.capnp";

struct Input {
    txid @0 : Data;
    n @1: UInt32;
    operation @2: Operation;

    enum Operation {
        transferAsset @0;
        evmCall @1;
    }
}

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

struct Address {
    union {
        blockHole @0: Void;
        eth @1: Data;
        fra @2: Data;
    }
}

struct Output {
    address @0: Address;

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
        evmCall @14: Evm.EvmCall;
    }

}

struct DefineAsset {
    transferable @0: Bool;
    maximum :union {
        none @1: Void;
        some @2: Data;
    }
}

struct RangeProof {
    rangeProof @0: Data;
    diffCommitmentLow @1: Data;
    diffCommitmentHigh @2: Data;
}

struct ChaumPedersenProof {
    c3 @0: Data;
    c4 @1: Data;
    z1 @2: Data;
    z2 @3: Data;
    z3 @4: Data;
}

struct ConfidentialAll {
    amount @0: RangeProof;
    asset @1: List(ChaumPedersenProof);
}

struct FraSignature {
    address @0: Address;
    publicKey @1: Data;
    siganture @2: Data;
}

struct Signature {
    union {
        fra @0: FraSignature;
        none @1: Void;
    }
}

struct Transaction {
    txid @0: Data;
    inputs @1: List(Input);
    outputs @2: List(Output);
    signature @3: List(Signature);
    proof :union {
        assetMix @4: Data;
        confidentialAmount @5: RangeProof;
        confidentialAsset @6: List(ChaumPedersenProof);
        confidentialAll @7: ConfidentialAll;
        noProof @8: Void;
    }
}

