<script lang="ts">
  import "fluent-svelte/theme.css";
  import "./style.css";
  import "../app.css";

  import Navbar from "$lib/Navbar/Navbar.svelte";
  import { onMount } from "svelte";
  import { isWin11 } from "$lib/commands/os";
  import { os, path } from "@tauri-apps/api";

  function add_drag_region() {
    const targetNode = window.document.body;

    const observer = new MutationObserver((mutationsList, observer) => {
      for (const mutation of mutationsList) {
        if (mutation.type === "childList" && mutation.addedNodes.length > 0) {
          const addedNode = mutation.addedNodes[0] as HTMLElement;
          if (addedNode.classList) {
            if (addedNode.classList.contains("content-dialog-smoke")) {
              addedNode.attributes.setNamedItem;
              addedNode.setAttribute("data-tauri-drag-region", "");
            }
          }
        }
      }
    });

    const config = { childList: true, subtree: true };

    observer.observe(targetNode, config);
  }

  onMount(async () => {
    add_drag_region();
    var styles = "";
    if (await isWin11()) {
      styles = `
:root {
	background: transparent;
}`;
    } else if ((await os.platform()) == "darwin") {
      styles = `
		
:root {
	background-color: #f0f3f9aa; 
}

@media (prefers-color-scheme: dark) {
	:root {
			background-color: #221f22aa;
	}
}`;
    } else {
      styles = `
		
:root {
	background-color: #f0f3f9; 
}

@media (prefers-color-scheme: dark) {
	:root {
			background-color: #221f22;
	}
}`;
    }

    var styleSheet = document.createElement("style");
    styleSheet.innerText = styles;
    document.head.appendChild(styleSheet);
  });
</script>

{#if window.location.pathname != "/about"}
  <Navbar />
{/if}

<main class="h-full">
  <slot />
</main>
