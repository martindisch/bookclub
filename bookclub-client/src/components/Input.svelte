<script lang="ts">
  export let label: string;
  export let id: string;
  export let rows: number = 1;
  export let value: string | null = null;
</script>

<div class="flex flex-col gap-2">
  <label for={id}>{label}</label>
  {#if rows === 1}
    <input
      type="text"
      bind:value
      {id}
      class="bg-transparent rounded p-1 outline-solid-gray-700 focus:outline-solid-gray-800"
      required
    />
  {:else}
    <textarea
      {rows}
      bind:value
      {id}
      class="bg-transparent rounded p-1 outline-solid-gray-700 focus:outline-solid-gray-800"
      required
    />
  {/if}
</div>
