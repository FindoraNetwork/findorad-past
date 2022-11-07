@0x9edb1e9495ce4d5a;

using Memo = import "memo.capnp";
using Input = import "input.capnp";
using Output = import "output.capnp";
using Address = import "address.capnp";

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
    address @0: Address.Address;
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
    inputs @1: List(Input.Input);
    outputs @2: List(Output.Output);
    signature @3: List(Signature);
    memo @4: Memo.Memo;
    proof :union {
        assetMix @5: Data;
        confidentialAmount @6: RangeProof;
        confidentialAsset @7: List(ChaumPedersenProof);
        confidentialAll @8: ConfidentialAll;
        noProof @9: Void;
    }
}

