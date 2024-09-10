<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';
  import { appConfigDir } from '@tauri-apps/api/path';
  import {onMount} from 'svelte';
  import LoginScreen from '$lib/LoginScreen.svelte';
  import SetupScreen from '$lib/SetupScreen.svelte';
  import {exists, readTextFile} from '@tauri-apps/plugin-fs';
  import LoadingScreen from '$lib/LoadingScreen.svelte';

  let authenticated = false;
  let configured = false;
  let loading = true;

  const select_loadfile = async () => {
    const path = await open({
     filters: [{ name: "Loadfile", extensions: ['dat', 'csv']}]
    });
    const details = await invoke("open_loadfile", { path });
  }

  onMount(async() => {
    try {
      const appConfigDirPath = await appConfigDir();
      const cf = `${appConfigDirPath}/app.conf`;
      if (await exists(cf)) {
        const instance_uri = await readTextFile(cf);

        await invoke('verify_instance_uri', {instance_uri});
        configured = true;
      } else {
        configured = false;
      }
    } catch(e) {
      console.log(e);
    } finally {
      loading = false;
    }
  })
</script>

<div>
  {#if loading}
    <LoadingScreen/>
  {:else if !configured}
    <SetupScreen on:complete={() => configured = true}/>
  {:else if !authenticated}
    <LoginScreen/>
  {:else}
    <slot />
  {/if}
</div>

