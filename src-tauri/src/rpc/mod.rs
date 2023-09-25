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
        .start_http(&"0.0.0.0:8340".parse().unwrap())
        .unwrap();

    let close = server.close_handle().clone();
    let mut rpc_closer = RPC_CLOSER.lock().unwrap();
    *rpc_closer = Some(close);
    log::info!("rpc server started!");
    tokio::spawn(async { server.wait() });
}

#[rpc]
pub trait Rpc {
    // #[rpc(name = "deploy")]
    // fn deploy(
    //     private_key: String,
    //     program_id: String,
    //     path: Option<String>,
    //     record: String,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String>;

    // #[rpc(name = "execute")]
    // fn execute(
    //     private_key: String,
    //     program_id: String,
    //     function: String,
    //     inputs: Vec<String>,
    //     record: Option<String>,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String>;

    // #[rpc(name = "transfer")]
    // fn transfer(
    //     private_key: String,
    //     recipient: String,
    //     amount: u64,
    //     function: String,
    //     input_record: Option<String>,
    //     fee_record: Option<String>,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgramOutput {
    pub function: String,
    pub name: String,
    pub idx: usize,
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    // fn deploy(
    //     private_key: String,
    //     program_id: String,
    //     path: Option<String>,
    //     record: String,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String> {
    //     todo!();
    // }

    // fn execute(
    //     private_key: String,
    //     program_id: String,
    //     function: String,
    //     inputs: Vec<String>,
    //     record: Option<String>,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String> {
    //     todo!();
    // }

    // fn transfer(
    //     private_key: String,
    //     recipient: String,
    //     amount: u64,
    //     function: String,
    //     input_record: Option<String>,
    //     fee_record: Option<String>,
    //     fee: Option<u64>,
    //     query: Option<String>,
    // ) -> Result<String> {
    //     todo!();
    // }
}

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
