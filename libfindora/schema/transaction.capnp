@0x9edb1e9495ce4d5a;

struct Input {
    txid @0 : Data;
    n @1: UInt32;
    operation @2: Operation;
    signature @3: Data;

    enum Operation {
        transferAsset @0;
        issueAsset @1;
    }
}

struct ConfidentialAmount {
    point0 @0: Data;
    point1 @1: Data;
}

struct Output {
    publicKey @0: Data;

    operation @1: Operation;

    enum Operation {
        transferAsset @0;
        issueAsset @1;
    }

    amount :union {
        confidential @2: ConfidentialAmount;
        nonConfidential @3: UInt64;
    }

    asset :union {
        confidential @4: Data;
        nonConfidential @5: Data;
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

struct Transaction {
    txid @0: Data;
    inputs @1: List(Input);
    outputs @2: List(Output);
    proof :union {
        assetMix @3: Data;
        confidentialAmount @4: RangeProof;
        confidentialAsset @5: List(ChaumPedersenProof);
        confidentialAll @6: ConfidentialAll;
        noProof @7: Void;
    }
}

