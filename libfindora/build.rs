fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/evm.capnp")
        .file("schema/transaction.capnp")
        .run()
        .expect("schema compiler command");
}
