@0x9edb1e9495ce4d5a;

struct Input {
    txid @0 : Data;
    n @1: UInt32;
    operation @2: Operation;

    enum Operation {
        transferAsset @0;
        issueAsset @1;
        undelegate @2;
        claimReward @3;
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
        ed25519 @0: Data;
        secp256k1 @1: Data;
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

struct FraAddress {
    address @0: Data;
    publicKey :union {
        none @1: Void;
        some @2: Data;
    }
}

struct Output {
    address :union {
        eth @0: Data;
        fra @1: FraAddress;
    }

    amount :union {
        confidential @2: ConfidentialAmount;
        nonConfidential @3: UInt64;
    }

    asset :union {
        confidential @4: Data;
        nonConfidential @5: Data;
    }

    ownerMemo :union {
        none @6: Void;
        some @7: OwnerMemo;
    }

    operation :union {
        transferAsset @8: Void;
        issueAsset @9: Void;
        fee @10: Void;
        undelegate @11: UndelegateData;
        claimReward @12: ClaimData;
        delegate @13: DelegateData;
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
    publicKey @0: Data;
    siganture @1: Data;
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

