<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import Generate from "./generate.svelte";
  import { onMount } from 'svelte'
  import { themeChange } from 'theme-change'

  onMount(() => {
    themeChange(false)
  });

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
    if (!Array.isArray(selected) && selected != null) {
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

<div class="bg-base-300 flex flex-col min-h-screen">
  <div class="navbar text-white">
    <div class="navbar-start">
      {#if generate_page}
        <button class="btn btn-ghost btn-circle" on:click={() => generate_page = false}>
          <!-- <svg stroke="currentColor" class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
            <path  d="M8.388,10.049l4.76-4.873c0.303-0.31,0.297-0.804-0.012-1.105c-0.309-0.304-0.803-0.293-1.105,0.012L6.726,9.516c-0.303,0.31-0.296,0.805,0.012,1.105l5.433,5.307c0.152,0.148,0.35,0.223,0.547,0.223c0.203,0,0.406-0.08,0.559-0.236c0.303-0.309,0.295-0.803-0.012-1.104L8.388,10.049z"></path>
          </svg> -->
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-8 h-8">
            <path fill-rule="evenodd" d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z" clip-rule="evenodd" />
          </svg>
        </button>
      {/if}
    </div>
    <div class="navbar-center">
      <h1 class="btn btn-ghost normal-case text-3xl font-bold">prismatica</h1>
    </div>
    <div class="navbar-end">
      <button class="btn btn-ghost btn-circle" data-toggle-theme="rosepine-moon,rosepine-dawn">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-6 h-6">
          <path d="M15.993 1.385a1.87 1.87 0 012.623 2.622l-4.03 5.27a12.749 12.749 0 01-4.237 3.562 4.508 4.508 0 00-3.188-3.188 12.75 12.75 0 013.562-4.236l5.27-4.03zM6 11a3 3 0 00-3 3 .5.5 0 01-.72.45.75.75 0 00-1.035.931A4.001 4.001 0 009 14.004V14a3.01 3.01 0 00-1.66-2.685A2.99 2.99 0 006 11z" />
        </svg>
      </button>
    </div>
  </div>
  
  <div class="rounded-3xl w-full bg-base-100 flex-grow flex flex-col">
    
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
            <option value="rose_pine">Rose Pine</option>
          </select>
          <button class="btn btn-block" on:click={ask_for_file}>Choose Image</button>
          {#if file}
            <p class="mt-5">Chose: {file}</p>
          {/if}
          <button class="btn btn-block mt-10" on:click={generate}>Generate</button>
        </div>
      </div>
    {/if}

  </div>
</div>