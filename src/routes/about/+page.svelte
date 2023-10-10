<script lang="ts">
  import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";

  let TauriVersion;
  let version;
  onMount(async () => {
    TauriVersion = await getTauriVersion();
    version = await getVersion();
  });
</script>

<main class="container">
  <div class="column">
    <h1>Aleo wallet Acceleration Service</h1>
    <p>version: {version}</p>
    <p>tauri version: {TauriVersion}</p>
    <p>
      sourcecode: <a
        href="https://github.com/Aleo123Dev/aleo-acceleration-service"
        >https://github.com/Aleo123Dev/aleo-acceleration-service</a
      >
    </p>
  </div>
</main>
