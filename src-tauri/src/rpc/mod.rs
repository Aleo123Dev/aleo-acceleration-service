pub mod aes;
pub mod middleware;
pub mod rpc;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Mutex,
};

use lazy_static::lazy_static;
use tokio::sync::oneshot::Sender;

use crate::config::consts::RPC_PORT;

lazy_static! {
    static ref RPC_CLOSER: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

#[tauri::command]
pub fn stop_rpc_server() {
    let mut rpc_closer = RPC_CLOSER.lock().unwrap();
    match rpc_closer.take() {
        Some(v) => {
            let _ = v.send(());
            *rpc_closer = None;
        }
        None => {}
    }
}

#[tauri::command]
pub fn run_rpc_server() {
    tokio::spawn(async {
        #[cfg(debug_assertions)]
        let address: SocketAddr =
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), RPC_PORT));

        #[cfg(not(debug_assertions))]
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), RPC_PORT));

        let close = middleware::start_hyper(&address).await;
        let mut rpc_closer = RPC_CLOSER.lock().unwrap();
        *rpc_closer = Some(close);
        log::info!("rpc server started!");
    });
}
