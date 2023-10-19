<script lang="ts">
  import { get_server_url } from "$lib/commands/app";
  import { os_info, type Info } from "$lib/commands/os";
  import { Button } from "fluent-svelte-extra";
  import { onMount } from "svelte";

  import Copy_Regular from "svelte-fluentui-icons/icons/Copy_Regular.svelte";
  import SetPassDialog from "./set_pass_dialog.svelte";
  import { clipboard, tauri } from "@tauri-apps/api";

  let server_url;
  let osinfo: Info;

  let showpassdialog = false;

  onMount(async () => {
    server_url = await get_server_url();
    osinfo = await os_info();
  });
</script>

<SetPassDialog bind:open={showpassdialog} />
<div>
  <div>
    <h2>Server url:</h2>
  </div>
  <div class="flex my-4">
    <p class="break-all mx-5">{server_url}</p>
    <Button
      variant="hyperlink"
      on:click={async () => {
        clipboard.writeText(server_url);
      }}
    >
      <Copy_Regular alt="copy" size="35" />
    </Button>
  </div>

  <div class="my-4">
    <h2>Password</h2>
  </div>
  <div class="mx-5">
    <Button variant="accent" on:click={() => (showpassdialog = true)}
      >reset password</Button
    >
  </div>
  <div class="my-4">
    <h2>System:</h2>
  </div>
  <div class="mx-5">
    {#if osinfo}
      <p>type: {osinfo.os_type}</p>
      <p>edition: {osinfo.edition}</p>
      <p>version: {JSON.stringify(osinfo.version)}</p>
      <p>bitness: {osinfo.bitness}</p>
      <p>architecture: {osinfo.architecture}</p>
    {/if}
  </div>
</div>
