@0xbea30f40fd67d90d;

struct Create {
    salt @0 :Data;
}

struct Input {
    n @0: UInt32;
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
    data @3: Data;
    action @4: Action;
}


