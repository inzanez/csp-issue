<script lang="ts">
    import { fade } from 'svelte/transition';
    import {type ComponentType, createEventDispatcher} from 'svelte';
    import {invoke} from '@tauri-apps/api/core';
    import {appConfigDir} from '@tauri-apps/api/path';
    import {mkdir , exists, writeTextFile} from '@tauri-apps/plugin-fs';

    const dispatch = createEventDispatcher();

    let instance_field: ComponentType;
    let instance_uri: string;

    let loading = false;

    const instance_rules = [
        (v) => !!v || 'Required',
        (v) => {
            const pattern =
                /^(^$|(http(s)?:\/\/)([\w-]+\.)+[\w-]+([\w- ;,.\/?%&=]*))$/;
            return pattern.test(v) || 'Invalid instance address';
        }
    ];

    const run_setup = async () => {
        if (instance_field.validate()) {
            return;
        } else {
            try {
                loading = true;
                await invoke('verify_instance_uri', { instance_uri });
                const appConfigDirPath = await appConfigDir();

                console.log(appConfigDirPath);
                if(!await exists(appConfigDirPath)) {
                    await mkdir(appConfigDirPath, { recursive: true });
                }

                const cf = `${appConfigDirPath}/app.conf`;
                await writeTextFile(cf, instance_uri);
                dispatch('complete');
            } catch (e) {
                console.log('Setup failed', e);
            } finally {
                loading = false;
            }
        }
    };


</script>

<div in:fade class="setup-wrapper">

</div>

<style>


    .setup-wrapper {
        background-color: rgb(250, 250, 250);
        justify-content: center;
        width: 100vw;
        height: 100vh;
    }
</style>