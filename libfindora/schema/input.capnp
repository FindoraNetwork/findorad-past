@0xd4adb68873e618f8;

struct InputOperation {
    transferAsset @0: Void;
}

struct Input {
    txid @0 : Data;
    n @1: UInt32;

    operation @2: InputOperation;
}
