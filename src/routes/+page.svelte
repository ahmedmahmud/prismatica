<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import Generate from "./generate.svelte";

  let theme = "";
  let file = "";
  let generate_page = false;

  const ask_for_file = async () => {
    // Open a selection dialog for directories
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Image',
        extensions: ['png', 'jpeg', 'jpg', 'webp']
      }]
    });
    if (Array.isArray(selected)) {
      // Not possible
    } else if (selected === null) {
      file = "";
    } else {
      file = selected;
    }
  };

  const generate = () => {
    if (theme && file) {
      generate_page = true
    }
    // Show error otherwise
    // ...
  }
</script>

{#if generate_page}
  <Generate chosen_theme={theme} file_path={file}/>
{:else}
  <div class="flex h-full justify-center items-center flex-grow">
    <div class="text-center mb-10">
      <!-- <p class="text-5xl mb-10 text-opacity-60 text-white">Transform image</p> -->
      <select
        class="select select-lg select-bordered w-full max-w-xs mb-10"
        bind:value={theme}
      >
        <option value="" disabled selected>Theme</option>
        <option value="catppuccin">Catppuccin</option>
      </select>
      <button class="btn btn-block" on:click={ask_for_file}>Choose Image</button>
      {#if file}
        <p class="mt-5">Chose: {file}</p>
      {/if}
      <button class="btn btn-block mt-10" on:click={generate}>Generate</button>
    </div>
  </div>
{/if}