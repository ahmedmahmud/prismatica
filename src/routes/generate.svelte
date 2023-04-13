<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  export let chosen_theme: string;
  export let file_path: string;

  interface Theme {
    palettes: string[];
    noise: string[];
  }

  const themes: { [key: string]: Theme } = {
    catppuccin: {
      palettes: ["latte", "frappe", "macchiato", "mocha"],
      noise: ["0", "1", "2", "3", "4"],
    },
  };

  let theme = themes[chosen_theme];
  let tabs = theme.palettes;
  let selected_tab = tabs[0];

  let switch_tab = (tab: string) => () => selected_tab = tab;

  let get_image = async (palette: String, noise: String) => {
    const data: ArrayBufferLike = await invoke("convert_and_blob", {
      file_path,
      palette,
      noise,
    });
    const array = new Uint8Array(data);
    const blob = new Blob([array], { type: "image/jpeg" });
    return { data, url: URL.createObjectURL(blob) };
  };

  let blobs = theme.palettes.map((p) => {
    let promises = theme.noise.map((n) => {
      return get_image(p, n);
    });
    return {
      palette: p,
      images: promises,
    };
  });

  const download_image = (blob: ArrayBufferLike, palette: String, noise: String) => () => {
    invoke("save_from_blob", { blob, file_path, palette, noise });
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
  {#each blobs as blob_g}
    {#if selected_tab === blob_g.palette}
      {#each blob_g.images as promise, n}
        <div class="pb-20 break-inside-avoid">
          {#await promise}
            <p>...waiting</p>
          {:then { data, url }}
            <img src={url} class="mb-5 rounded-lg shadow-xl mx-auto" alt="" />
            <div class="flex gap-5">
              <button class="btn btn-primary flex-1 shadow-xl" on:click={download_image(data, blob_g.palette, n.toString())}>Download</button>
              <label class="btn btn-secondary flex-1 shadow-xl" for={`${blob_g.palette}-${n}`}>View</label>
            </div>
            <input type="checkbox" id={`${blob_g.palette}-${n}`} class="modal-toggle" />
            <label for={`${blob_g.palette}-${n}`} class="modal cursor-pointer p-10">
              <img src={url} class="rounded-lg shadow-xl max-h-full" alt="" />
            </label>
          {:catch error}
            <p style="color: red">{error.message}</p>
          {/await}
        </div>
      {/each}
    {/if}
  {/each}
</div>
