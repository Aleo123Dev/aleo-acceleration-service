use std::sync::Mutex;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{CloseHandle, ServerBuilder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RPC_CLOSER: Mutex<Option<CloseHandle>> = Mutex::new(None);
}

pub fn stop_rpc_server() {
    let mut rpc_closer = RPC_CLOSER.lock().unwrap();
    match rpc_closer.take() {
        Some(v) => v.close(),
        None => {}
    }
}

pub fn run_rpc_server() {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(8)
        .start_http(&"0.0.0.0:18340".parse().unwrap())
        .unwrap();

    let close = server.close_handle().clone();
    let mut rpc_closer = RPC_CLOSER.lock().unwrap();
    *rpc_closer = Some(close);
    log::info!("rpc server started!");
    tokio::spawn(async { server.wait() });
}

#[rpc]
pub trait Rpc {
    #[rpc(name = "deploy")]
    fn deploy(
        private_key: String,
        program_id: String,
        path: String,
        record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        aleowrap::deploy(private_key, program_id, path, record, fee, query)
    }

    #[rpc(name = "execute")]
    fn execute(
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        aleowrap::execute(
            private_key,
            program_id,
            function,
            inputs,
            record,
            fee,
            query,
        )
    }

    #[rpc(name = "transfer")]
    fn transfer(
        private_key: String,
        recipient: String,
        amount: u64,
        function: String,
        input_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        aleowrap::transfer(
            private_key,
            recipient,
            amount,
            function,
            input_record,
            fee_record,
            fee,
            query,
        )
    }

    #[rpc(name = "join")]
    fn join(
        private_key: String,
        first_record: String,
        second_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        aleowrap::join(
            private_key,
            first_record,
            second_record,
            fee_record,
            fee,
            query,
        )
    }

    #[rpc(name = "split")]
    fn split(
        private_key: String,
        record: String,
        amount: u64,
        query: Option<String>,
    ) -> Result<String> {
        aleowrap::split(private_key, record, amount, query)
    }
}

pub struct RpcImpl;

impl Rpc for RpcImpl {}

pub fn to_jsonrpc_error(err: impl ToString) -> jsonrpc_core::error::Error {
    let mut error = jsonrpc_core::error::Error::new(jsonrpc_core::ErrorCode::ServerError(500));
    error.data = Some(serde_json::Value::String(err.to_string()));
    error
}

trait ToJsonRpcResult<T> {
    fn to_jsonrpc_result(self) -> jsonrpc_core::Result<T>;
}

impl<T> ToJsonRpcResult<T> for anyhow::Result<T> {
    fn to_jsonrpc_result(self) -> jsonrpc_core::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                let mut error =
                    jsonrpc_core::error::Error::new(jsonrpc_core::ErrorCode::ServerError(500));
                error.data = Some(serde_json::Value::String(err.to_string()));
                Err(error)
            }
        }
    }
}
