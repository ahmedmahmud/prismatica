<script>
  import "../app.css";
</script>

<div class="bg-base-300 flex flex-col min-h-screen">
  <div class="navbar text-white">
    <div class="navbar-start">
      <button class="btn btn-ghost btn-circle">
        <svg stroke="currentColor" class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
          <path  d="M8.388,10.049l4.76-4.873c0.303-0.31,0.297-0.804-0.012-1.105c-0.309-0.304-0.803-0.293-1.105,0.012L6.726,9.516c-0.303,0.31-0.296,0.805,0.012,1.105l5.433,5.307c0.152,0.148,0.35,0.223,0.547,0.223c0.203,0,0.406-0.08,0.559-0.236c0.303-0.309,0.295-0.803-0.012-1.104L8.388,10.049z"></path>
        </svg>
        <!-- <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg> -->
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