use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Mutex,
};

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{CloseHandle, ServerBuilder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::Instant;

lazy_static! {
    static ref RPC_CLOSER: Mutex<Option<CloseHandle>> = Mutex::new(None);
}

const RPC_PORT: u16 = 18340;

macro_rules! call_aleo_function {
    ($func:ident($($arg:expr),*)) => {
        {
            let start_time = Instant::now();
            log::info!(target: "aleosdk","executing method '{}'",stringify!($func));
            let result  = aleowrap::$func($($arg),*);
            let elapsed_time = Instant::now() - start_time;
            log::info!(target: "aleosdk","method '{}' took {} ms", stringify!($func),elapsed_time.as_millis());
            result
        }
    };
}

#[tauri::command]
pub fn stop_rpc_server() {
    let mut rpc_closer = RPC_CLOSER.lock().unwrap();
    match rpc_closer.take() {
        Some(v) => {
            v.close();
            *rpc_closer = None;
        }
        None => {}
    }
}

#[tauri::command]
pub fn run_rpc_server() {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());

    #[cfg(debug_assertions)]
    let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), RPC_PORT));

    #[cfg(not(debug_assertions))]
    let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), RPC_PORT));

    let server = ServerBuilder::new(io)
        .threads(8)
        .request_middleware(|q| jsonrpc_http_server::RequestMiddlewareAction::Proceed {
            should_continue_on_invalid_cors: true,
            request: q,
        })
        .start_http(&address)
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
        &self,
        private_key: String,
        program_id: String,
        path: String,
        record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "execute")]
    fn execute(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "transfer")]
    fn transfer(
        &self,
        private_key: String,
        recipient: String,
        amount: u64,
        function: String,
        input_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "join")]
    fn join(
        &self,
        private_key: String,
        first_record: String,
        second_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "split")]
    fn split(
        &self,
        private_key: String,
        record: String,
        amount: u64,
        query: Option<String>,
    ) -> Result<String>;

    #[rpc(name = "discovery")]
    fn discovery(&self) -> Result<Discovery>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discovery {
    version: String,
    features: Vec<String>,
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    fn deploy(
        &self,
        private_key: String,
        program_id: String,
        path: String,
        record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'deploy'");
        call_aleo_function!(deploy(
            &private_key,
            &program_id,
            &path,
            &record,
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("deploy")
    }

    fn execute(
        &self,
        private_key: String,
        program_id: String,
        function: String,
        inputs: Vec<String>,
        record: Option<String>,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'execute'");
        call_aleo_function!(execute(
            &private_key,
            &program_id,
            &function,
            inputs,
            record.as_deref(),
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("execute")
    }

    fn transfer(
        &self,
        private_key: String,
        recipient: String,
        amount: u64,
        function: String,
        input_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'transfer'");
        call_aleo_function!(transfer(
            &private_key,
            &recipient,
            amount,
            &function,
            &input_record,
            &fee_record,
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("transfer")
    }

    fn join(
        &self,
        private_key: String,
        first_record: String,
        second_record: String,
        fee_record: String,
        fee: Option<u64>,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'join'");
        call_aleo_function!(join(
            &private_key,
            &first_record,
            &second_record,
            &fee_record,
            fee,
            query.as_deref()
        ))
        .to_jsonrpc_result()
        .log_rpc_error("join")
    }

    fn split(
        &self,
        private_key: String,
        record: String,
        amount: u64,
        query: Option<String>,
    ) -> Result<String> {
        log::info!(target: "rpc","executing rpc method 'split'");
        call_aleo_function!(split(&private_key, &record, amount, query.as_deref()))
            .to_jsonrpc_result()
            .log_rpc_error("split")
    }

    fn discovery(&self) -> Result<Discovery> {
        log::info!(target: "rpc","executing rpc method 'discovery'");
        Ok(Discovery {
            version: env!("CARGO_PKG_VERSION").to_string(),
            features: vec![
                "deploy".to_string(),
                "execute".to_string(),
                "transfer".to_string(),
                "join".to_string(),
                "split".to_string(),
            ],
        })
    }
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
                let error = to_jsonrpc_error(err);
                Err(error)
            }
        }
    }
}

trait RpcLog<T> {
    fn log_rpc_error(self, method: &str) -> jsonrpc_core::Result<T>;
}

impl<T> RpcLog<T> for jsonrpc_core::Result<T> {
    fn log_rpc_error(self, method: &str) -> jsonrpc_core::Result<T> {
        if self.is_err() {
            let err = self.as_ref().err().unwrap().clone();
            log::error!(target: "rpc error", "method: {} ,code:{}, msg: {:?}",method, err.code.description(), err.message);
        }
        self
    }
}
