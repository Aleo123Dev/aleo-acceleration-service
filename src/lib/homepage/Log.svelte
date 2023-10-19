<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onDestroy, onMount } from "svelte";

  let logs = "";
  let timerId: number;

  onMount(async () => {
    timerId = setInterval(() => {
      greet();
    }, 100);
  });

  onDestroy(() => {
    clearInterval(timerId);
  });

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // greetMsg = await invoke("greet", { })
    logs = await invoke("get_logs", {});
  }
</script>

<div style="width: 90%;">
  <div style="text-align: left;">
    {#each logs as log_msg}
      <p>{log_msg}</p>
    {/each}
  </div>
</div>
