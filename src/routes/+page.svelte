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

<div class="tabs tabs-boxed flex-initial">
  <a class="tab flex-1">Latte</a> 
  <a class="tab tab-active flex-1">Frappe</a> 
  <a class="tab flex-1">Macchiato</a>
  <a class="tab flex-1">Mocha</a>
</div>

<div class="columns-md gap-20 p-20">
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

<!-- Put this part before </body> tag -->
<input type="checkbox" id="my-modal-4" class="modal-toggle" />
<label for="my-modal-4" class="modal cursor-pointer p-10">
  <img src={url} class="rounded-lg shadow-xl max-h-full" alt="">

  <!-- <label class="modal-box relative" for="">
    <h3 class="text-lg font-bold">Congratulations random Internet user!</h3>
    <p class="py-4">You've been selected for a chance to get one year of subscription to use Wikipedia for free!</p>
  </label> -->
</label>