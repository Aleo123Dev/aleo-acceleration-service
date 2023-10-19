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

  export let open: boolean = false;

  let newpassword = "";
  let newpassword_confirm = "";

  let errmsg = null;

  async function submit() {
    try {
      if (newpassword == newpassword_confirm) {
        await set_password(newpassword);
      } else {
        throw "password not match!";
      }
      open = false;
    } catch (e) {
      errmsg = e;
    }
  }
</script>

<ContentDialog bind:open>
  <h2 data-tauri-drag-region class="text-xl mb-4">set password</h2>
  <p>
    set a password to protect your account, leave the input to blank if you dont
    need a password.
  </p>

  <div>
    <p class="my-2">set password</p>
    <TextBox type="text" bind:value={newpassword} on:input />
    <p class="my-2">confirm password</p>
    <TextBox bind:value={newpassword_confirm} />
  </div>
  {#if errmsg}
    <p>{errmsg}</p>
  {/if}
  <svelte:fragment slot="footer">
    <Button variant="accent" on:click={submit}>Submit</Button>
  </svelte:fragment>
</ContentDialog>
