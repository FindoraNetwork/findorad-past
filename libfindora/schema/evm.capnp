@0xbea30f40fd67d90d;

struct Create {
    salt @0 :Data;
}

struct EvmCall {
    nonce @0 : UInt64;
    gasLimit @1: UInt64;
    data @2: Data;
    action :union {
        call @3: Void;
        create @4: Void;
        create2 @5: Create;
    }
}


