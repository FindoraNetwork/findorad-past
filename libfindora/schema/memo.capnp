@0xeda9951b5bd11cc6;

struct Ethereum {
    data @0: Data;
    n @1: UInt32;
}

struct Memo {
    ethereum @0: List(Ethereum);
}

