<script lang="ts">
  import {
    ContentDialog,
    Button,
    ListItem,
    TextBox,
    TextBoxButton,
  } from "fluent-svelte-extra";

  import Folder_Regular from "svelte-fluentui-icons/icons/Folder_Regular.svelte";
  import { open as opendialog } from "@tauri-apps/api/dialog";
  import {
    input_password,
    set_password,
    has_password,
  } from "$lib/commands/password";
  import { onMount } from "svelte";

  export let onsubmit: () => void | Promise<void>;
  export let open: boolean = false;

  let password = "";
  let newpassword = "";
  let newpassword_confirm = "";

  let should_set_password = true;

  let errmsg = null;

  onMount(async () => {
    should_set_password = !(await has_password());
  });

  async function submit() {
    try {
      if (should_set_password) {
        if (newpassword == newpassword_confirm) {
          await set_password(newpassword);
        } else {
          throw "password not match!";
        }
      } else {
        await input_password(password);
      }
      onsubmit();
      open = false;
    } catch (e) {
      errmsg = e;
    }
  }
</script>

<ContentDialog bind:open>
  {#if should_set_password}
    <h2 data-tauri-drag-region class="text-xl mb-4">set password</h2>
    <p>
      set a password to protect your account, leave the input to blank if you
      dont need a password.
    </p>
  {:else}
    <h2 data-tauri-drag-region class="text-xl mb-4">unlock app</h2>
  {/if}

  <div>
    {#if should_set_password}
      <p class="my-2">set password</p>
      <TextBox type="text" bind:value={newpassword} on:input />
      <p class="my-2">confirm password</p>
      <TextBox bind:value={newpassword_confirm} />
    {:else}
      <p class="my-2">app password</p>
      <TextBox bind:value={password} />
    {/if}
  </div>
  {#if errmsg}
    <p>{errmsg}</p>
  {/if}
  <svelte:fragment slot="footer">
    <Button variant="accent" on:click={submit}>Submit</Button>
    {#if !should_set_password}
      <Button on:click={() => (should_set_password = true)}
        >Reset password</Button
      >
    {/if}
  </svelte:fragment>
</ContentDialog>
