<script>
  import "../app.css";
</script>

<div class="bg-base-300 flex flex-col min-h-screen">
  <div class="navbar text-white">
    <div class="navbar-start">
      <button class="btn btn-ghost btn-circle">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
      </button>
    </div>
    <div class="navbar-center">
      <h1 class="btn btn-ghost normal-case text-xl">prismatica</h1>
    </div>
    <div class="navbar-end" />
  </div>
  
  <div class="rounded-3xl w-full bg-base-100 flex-grow flex flex-col">
    <slot />
  </div>
</div>