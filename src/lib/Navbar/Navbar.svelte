<script lang="ts">
  import "./Navbar.css";
  import logo from "$lib/logo/logo.png";

  import { appWindow } from "@tauri-apps/api/window";
  import Maximize_Regular from "svelte-fluentui-icons/icons/Maximize_Regular.svelte";
  import LineHorizontal1_Regular from "svelte-fluentui-icons/icons/LineHorizontal1_Regular.svelte";
  import Dismiss_Regular from "svelte-fluentui-icons/icons/Dismiss_Regular.svelte";
  import { IconButton } from "fluent-svelte";

  import { os } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let displaytitlebar = true;

  onMount(async () => {
    let platform = await os.platform();
    if (platform == "darwin") {
      displaytitlebar = false;
    }
  });
</script>

<header class="navbar">
  <div data-tauri-drag-region class="grow flex mt-1 mx-1">
    <a class="pt-3 mx-3" href="/">
      <div style="display: flex; align-items: center;" class="underline">
        <img src={logo} class="px-2 py-2 h-14 w-14" alt="App Logo" />
        <p class="title-text">Aleo Acceleration Service</p>
      </div>
    </a>
    <div data-tauri-drag-region class="grow" />
  </div>

  {#if displaytitlebar}
    <div data-tauri-drag-region class="titlebar flex justify-end">
      <IconButton
        class="w-12 h-8"
        id="titlebar-minimize"
        on:click={() => appWindow.minimize()}
      >
        <LineHorizontal1_Regular alt="minimize" />
      </IconButton>
      <IconButton
        class="w-12 h-8"
        id="titlebar-maximize"
        on:click={() => appWindow.toggleMaximize()}
      >
        <Maximize_Regular alt="maximize" />
      </IconButton>
      <IconButton
        class="w-12 h-8"
        id="titlebar-close"
        on:click={() => appWindow.close()}
      >
        <Dismiss_Regular alt="close" />
      </IconButton>
    </div>
  {/if}
</header>
