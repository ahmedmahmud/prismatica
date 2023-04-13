<script>
  // When using the Tauri API npm package:
  import { invoke } from '@tauri-apps/api'

  let url = ""

  let get_image = async () => {
    const data = await invoke('get_image');
    const array = new Uint8Array(data);
    const blob = new Blob([array], { type: 'image/jpeg' });
    url = URL.createObjectURL(blob);
  }
  
  get_image()
  get_image()
  get_image()
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if url}
  <img src="{url}" />
{/if}
