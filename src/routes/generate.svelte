<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from "@tauri-apps/api";
  import p_queue from 'p-queue';

  export let chosen_theme: string;
  export let file_path: string;
  
  interface InputImage {
    path: string,
    stem: string,
    ext: string,
    theme: string,
    palette: string,
    noise: string,
  } 
  
  interface OutputImage {
    source: InputImage;
    blob: number[];
  }
  
  interface Result {
    url: string
    data: OutputImage
  }

  const revoke_images = () => {
    queue.clear();
    for (const palette of Object.values(cache).flatMap(p => p ? [p]: [])) {
      for (const result of palette.flatMap(r => r ? [r] : [])) {
        URL.revokeObjectURL(result.url);
      }
    }
  };
  
  const noise_levels = ["0", "1", "2", "3", "4"];
  
  const themes: { [key: string]: string[] } = {
    catppuccin: ["latte", "frappe", "macchiato", "mocha", "oled"],
    rose_pine: ["moon"]
  };
  
  let palettes = themes[chosen_theme];
  let selected_tab = palettes[0];
  
  const queue = new p_queue({ concurrency: 5 });
  
  let cache = palettes.reduce((acc, p) => {
    acc[p] = undefined;
    return acc;
  }, {} as Record<string, (Result | null)[] | undefined>);
  
  onDestroy(revoke_images);

  let switch_tab = (tab: string) => () => selected_tab = tab;
  
  let get_image = async (palette: string, noise: string) => {
    console.log("fetching", palette, noise);
    const data: OutputImage = await invoke("convert", {
      path: file_path,
      theme: chosen_theme,
      palette,
      noise,
    });

    const array = new Uint8Array(data.blob);
    const blob = new Blob([array], { type: `image/png` });
    return { data, url: URL.createObjectURL(blob) };
  };

  $: {
    let tab = selected_tab;
    
    if (cache[tab] === undefined) {
      let result: (Result | null)[] = noise_levels.map(() => null);
      cache[tab] = result;

      for (const noise of noise_levels) {
        (async () => {
          let data = await queue.add(() => get_image(tab, noise));
          if (data) {
            result[+noise] = data;
            cache[tab] = result;
          }
        })();
      }
    }
  }

  const download_image = (image: OutputImage) => () => {
    invoke("save_output", { image });
  }
</script>

<div class="flex place-content-center mt-10">
  <div class="tabs tabs-boxed inline-flex">
    {#each palettes as tab}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="tab sm:tab-lg {selected_tab === tab ? 'tab-active' : ''}" on:click={switch_tab(tab)}>
        {tab.charAt(0).toUpperCase() + tab.slice(1)}
      </div>
    {/each}
  </div>
</div>

<div class="columns-md gap-20 py-10 px-20">
  {#each cache[selected_tab] || [] as image}
    <div class="pb-20 break-inside-avoid">
    {#if !image}
    <div class="mx-auto border-t-transparent border-solid animate-spin rounded-full border-neutral-content border-4 h-36 w-36 mb-10"></div>
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl" disabled>Download</button>
      <button class="btn btn-secondary flex-1 shadow-xl" disabled>View</button>
    </div>
    {:else}
    <img src={image.url} class="mb-5 rounded-lg shadow-xl mx-auto" alt="" />
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl" on:click={download_image(image.data)}>Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for={`${image.data.source.palette}-${image.data.source.noise}`}>View</label>
    </div>
    <input type="checkbox" id={`${image.data.source.palette}-${image.data.source.noise}`} class="modal-toggle" />
    <label for={`${image.data.source.palette}-${image.data.source.noise}`} class="modal cursor-pointer p-10">
      <img src={image.url} class="rounded-lg shadow-xl max-h-full" alt="" />
    </label>
    {/if}
    </div>
  {/each}
</div>
