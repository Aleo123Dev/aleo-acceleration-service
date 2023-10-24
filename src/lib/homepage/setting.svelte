<script lang="ts">
  import { get_server_url } from "$lib/commands/app";
  import { os_info, type Info } from "$lib/commands/os";
  import { Button, TextBox } from "fluent-svelte";
  import { onMount } from "svelte";

  import LinkMultiple_Regular from "svelte-fluentui-icons/icons/LinkMultiple_Regular.svelte";
  import CatchUp_Regular from "svelte-fluentui-icons/icons/CatchUp_Regular.svelte";
  import LockClosedKey_Regular from "svelte-fluentui-icons/icons/LockClosedKey_Regular.svelte";
  import Info_Regular from "svelte-fluentui-icons/icons/Info_Regular.svelte";

  import SetPassDialog from "./set_pass_dialog.svelte";
  import { clipboard, tauri } from "@tauri-apps/api";
  import { get_proxy, set_proxy } from "$lib/commands/config";
  import SetProxyDialog from "./set_proxy_dialog.svelte";

  let server_url;
  let osinfo: Info;

  let showpassdialog = false;
  let showproxydialog = false;
  let proxy = "";

  onMount(async () => {
    server_url = await get_server_url();
    osinfo = await os_info();
    proxy = await get_proxy();
  });
</script>

<SetPassDialog bind:open={showpassdialog} />
<SetProxyDialog
  bind:open={showproxydialog}
  onsubmit={async () => {
    proxy = await get_proxy();
  }}
/>
<div>
  <div class="setting">
    <div class="flex items-center">
      <LinkMultiple_Regular class="mr-2" />
      <p>Server url</p>
    </div>
    <Button
      variant="standard"
      on:click={async () => {
        clipboard.writeText(server_url);
      }}
    >
      copy
    </Button>
  </div>
  <div class="settingContent">
    <p class="break-all">{server_url}</p>
  </div>

  <div class="setting">
    <div class="flex items-center">
      <LockClosedKey_Regular class="mr-2" />
      <p>Password</p>
    </div>
    <Button variant="standard" on:click={() => (showpassdialog = true)}
      >reset password</Button
    >
  </div>

  <div class="setting">
    <div class="flex items-center">
      <CatchUp_Regular class="mr-2" />
      <div>
        <p>Proxy</p>
        <p class="text-xs">{proxy}</p>
      </div>
    </div>

    <Button
      variant="standard"
      on:click={() => {
        showproxydialog = true;
      }}>set http proxy</Button
    >
  </div>

  <div class="setting">
    <div class="flex items-center">
      <Info_Regular class="mr-2" />
      <p>System</p>
    </div>
    <Button
      variant="standard"
      on:click={() => clipboard.writeText(JSON.stringify(osinfo))}>copy</Button
    >
  </div>
  <div class="settingContent">
    {#if osinfo}
      <p>type: {osinfo.os_type}</p>
      <p>edition: {osinfo.edition}</p>
      <p>version: {JSON.stringify(osinfo.version)}</p>
      <p>bitness: {osinfo.bitness}</p>
      <p>architecture: {osinfo.architecture}</p>
    {/if}
  </div>
</div>

<style>
  .setting {
    height: 55px;
    @apply mt-2 px-4 card flex justify-between items-center;
  }

  .setting:hover {
    @apply card-hover;
  }

  .settingContent {
    @apply card px-12 py-4 text-sm;
  }
</style>
