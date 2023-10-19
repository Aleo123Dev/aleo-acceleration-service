import { invoke } from "@tauri-apps/api/tauri";

export async function run_rpc_server() {
    await invoke('run_rpc_server', {})
}

export async function stop_rpc_server() {
    await invoke('stop_rpc_server', {})
}

export async function get_server_url(): Promise<string> {
    return await invoke('get_server_url', {})
}