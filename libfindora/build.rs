fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/address.capnp")
        .file("schema/evm.capnp")
        .file("schema/memo.capnp")
        .file("schema/input.capnp")
        .file("schema/output.capnp")
        .file("schema/transaction.capnp")
        .run()
        .expect("schema compiler command");
}
