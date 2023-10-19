<script lang="ts">
  import { shell } from "@tauri-apps/api";
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";

  let TauriVersion;
  let version;
  onMount(async () => {
    TauriVersion = await getTauriVersion();
    version = await getVersion();
  });
</script>

<div class="column">
  <h1>Aleo wallet Acceleration Service</h1>
  <p>version: {version}</p>
  <p>tauri version: {TauriVersion}</p>
  <p>
    <!-- svelte-ignore a11y-missing-attribute -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    sourcecode:
    <a
      on:click={() =>
        shell.open("https://github.com/Aleo123Dev/aleo-acceleration-service")}
      >https://github.com/Aleo123Dev/aleo-acceleration-service</a
    >
  </p>
</div>
