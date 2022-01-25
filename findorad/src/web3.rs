#[cfg(any(feature = "web3-http", feature = "web3-ws"))]
pub fn strat_web3() {
    use web3_server::Web3ServerBuilder;

    // TODO: Add ws addr.
    let builder = Web3ServerBuilder {
        upstream: "http://127.0.0.1:26657".to_string(),
        http: "127.0.0.1:8545".parse().expect("parse socket address error."),
    };

    let server = builder.build();
    server.start();
}

#[cfg(not(any(feature = "web3-http", feature = "web3-ws")))]
pub fn strat_web3() {
    panic!("No compile with web3, please recompile with web3 feature.")
}
