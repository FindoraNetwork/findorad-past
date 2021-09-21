fn main() {
    capnpc::CompilerCommand::new()
        // .src_prefix("schema")
        .file("schema/transaction.capnp")
        .run().expect("schema compiler command");
}
