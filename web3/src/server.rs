use std::{net::SocketAddr, thread};

use crate::{EthApiImpl, NetApiImpl, Web3ApiImpl};
use web3_rpc_core::{EthApi, NetApi, Web3Api};

pub struct Web3ServerBuilder {
    pub upstream: String,
    #[cfg(feature = "http")]
    pub http: SocketAddr,
    #[cfg(feature = "ws")]
    pub ws: SocketAddr,
}

impl Web3ServerBuilder {
    #[cfg(feature = "http")]
    fn build_http(&self) -> jsonrpc_http_server::Server {
        let upstream = self.upstream.clone();

        let mut io = jsonrpc_core::IoHandler::new();

        let eth = EthApiImpl {
            upstream: upstream.clone(),
        };

        let net = NetApiImpl {
            upstream: upstream.clone(),
        };

        let web3 = Web3ApiImpl {
            upstream,
        };

        io.extend_with(eth.to_delegate());
        io.extend_with(net.to_delegate());
        io.extend_with(web3.to_delegate());

        jsonrpc_http_server::ServerBuilder::new(io.clone())
            .start_http(&self.http)
            .expect("failed to create http server")
    }

    #[cfg(feature = "ws")]
    fn build_ws(&self) -> jsonrpc_ws_server::Server {
        let upstream = self.upstream.clone();

        let mut io = jsonrpc_core::IoHandler::new();

        let eth = EthApiImpl {
            upstream: upstream.clone(),
        };

        let net = NetApiImpl {
            upstream: upstream.clone(),
        };

        let web3 = Web3ApiImpl {
            upstream,
        };

        io.extend_with(eth.to_delegate());
        io.extend_with(net.to_delegate());
        io.extend_with(web3.to_delegate());

        jsonrpc_ws_server::ServerBuilder::new(io.clone())
            .start(&self.ws)
            .expect("failed to create http server")
    }
    pub fn build(self) -> Web3Server {
        #[cfg(feature = "http")]
        let http = self.build_http();

        #[cfg(feature = "ws")]
        let ws = self.build_ws();

        Web3Server {
            #[cfg(feature = "http")]
            http,
            #[cfg(feature = "ws")]
            ws,
        }
    }
}

pub struct Web3Server {
    #[cfg(feature = "http")]
    http: jsonrpc_http_server::Server,
    #[cfg(feature = "ws")]
    ws: jsonrpc_ws_server::Server,
}

impl Web3Server {
    pub fn start(self) {
        #[cfg(feature = "http")]
        let _ = thread::spawn(move || {
            self.http.wait();
        });

        #[cfg(feature = "ws")]
        let _ = thread::spawn(move || {
            self.ws.wait().expect("ws start error");
        });
    }
}

