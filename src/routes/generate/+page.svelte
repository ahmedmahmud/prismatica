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
  
  get_image();
</script>

<div class="flex place-content-center mt-10">
  <div class="tabs tabs-boxed inline-flex">
    <div class="tab">Latte</div> 
    <div class="tab tab-active">Frappe</div> 
    <div class="tab">Macchiato</div>
    <div class="tab">Mocha</div>
  </div>
</div>

<div class="columns-md gap-20 py-10 px-20">
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
  <div class="pb-20 break-inside-avoid">
    <img src={url} class="mb-5 rounded-lg shadow-xl" alt="">
    <div class="flex gap-5">
      <button class="btn btn-primary flex-1 shadow-xl">Download</button>
      <label class="btn btn-secondary flex-1 shadow-xl" for="my-modal-4">View</label>
    </div>
  </div>
</div>

<label for="my-modal-4" class="modal cursor-pointer p-10">
  <img src={url} class="rounded-lg shadow-xl max-h-full" alt="">
</label>