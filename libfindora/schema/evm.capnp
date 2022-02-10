@0xbea30f40fd67d90d;

struct Create {
    salt @0 :Data;
}

struct Action {
    union {
        call @0: Void;
        create @1: Void;
        create2 @2: Create;
    }
}

struct Output {
    chainId @0 :UInt64;
    nonce @1 : UInt64;
    gasLimit @2: UInt64;
    gasPrice @3: UInt64;
    data @4: Data;
    action @5: Action;
    caller @6: Data;
}


