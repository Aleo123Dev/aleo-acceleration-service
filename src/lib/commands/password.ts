import { invoke } from "@tauri-apps/api/tauri";


export async function input_password(password: string) {
    await invoke('input_password', { password: password })
}

export async function set_password(password: string) {
    await invoke('set_password', { password: password })
}
export async function has_password(): Promise<boolean> {
    return await invoke('has_password', {})
}

export async function try_password(): Promise<boolean> {
    return await invoke('try_password', {})
}