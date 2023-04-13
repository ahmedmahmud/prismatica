<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  export let chosen_theme: string;
  export let file_path: string;

  interface Theme {
    palettes: string[];
    noise: string[];
  }

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

  const themes: { [key: string]: Theme } = {
    catppuccin: {
      palettes: ["latte", "frappe", "macchiato", "mocha", "oled"],
      noise: ["0", "1", "2", "3", "4"],
    },
  };

  let theme = themes[chosen_theme];
  let tabs = theme.palettes;
  let selected_tab = tabs[0];

  let switch_tab = (tab: string) => () => selected_tab = tab;

  let get_image = async (palette: String, noise: String) => {
    const data: OutputImage = await invoke("convert", {
      path: file_path,
      theme: "catppuccin",
      palette,
      noise,
    });

    const array = new Uint8Array(data.blob);
    const blob = new Blob([array], { type: "image/jpeg" });
    return { data, url: URL.createObjectURL(blob) };
  };

  $: images = theme.noise.map((n) => get_image(selected_tab, n));

  const download_image = (image: OutputImage) => () => {
    invoke("save_output", { image });
  }
</script>

<div class="flex place-content-center mt-10">
  <div class="tabs tabs-boxed inline-flex">
    {#each tabs as tab}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="tab {selected_tab === tab ? 'tab-active' : ''}" on:click={switch_tab(tab)}>
        {tab.charAt(0).toUpperCase() + tab.slice(1)}
      </div>
    {/each}
  </div>
</div>

<div class="columns-md gap-20 py-10 px-20">
  {#each images as image}
    <div class="pb-20 break-inside-avoid">
      {#await image}
        <div class="mx-auto border-t-transparent border-solid animate-spin rounded-full border-neutral-content border-4 h-36 w-36 mb-10"></div>
        <div class="flex gap-5">
          <button class="btn btn-primary flex-1 shadow-xl" disabled>Download</button>
          <button class="btn btn-secondary flex-1 shadow-xl" disabled>View</button>
        </div>
      {:then { data, url }}
        <img src={url} class="mb-5 rounded-lg shadow-xl mx-auto" alt="" />
        <div class="flex gap-5">
          <button class="btn btn-primary flex-1 shadow-xl" on:click={download_image(data)}>Download</button>
          <label class="btn btn-secondary flex-1 shadow-xl" for={`${data.source.palette}-${data.source.noise}`}>View</label>
        </div>
        <input type="checkbox" id={`${data.source.palette}-${data.source.noise}`} class="modal-toggle" />
        <label for={`${data.source.palette}-${data.source.noise}`} class="modal cursor-pointer p-10">
          <img src={url} class="rounded-lg shadow-xl max-h-full" alt="" />
        </label>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    </div>
  {/each}
</div>
