<script lang="ts">
  import "virtual:windi.css";
</script>

<svelte:head>
  <link rel="icon" href="/favicon.ico" sizes="any" />
  <link rel="icon" href="/icon.svg" type="image/svg+xml" />
  <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
  <link rel="manifest" href="/manifest.webmanifest" />
</svelte:head>

<div class="min-h-screen bg-gray-900 p-6 text-gray-100 flex justify-center">
  <div class="max-w-screen-md w-full"><slot /></div>
</div>
