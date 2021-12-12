<script lang="ts">
  export let label: string;
  export let id: string;
  export let value: number | null = null;
</script>

<div class="flex flex-col gap-2">
  <label for={id}>{label}</label>
  <input
    type="number"
    step="1"
    bind:value
    {id}
    class="bg-transparent rounded p-1 outline-solid-gray-700 focus:outline-solid-gray-800"
    required
  />
</div>

<style>
  /* Chrome, Safari, Edge, Opera */
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  /* Firefox */
  input[type="number"] {
    -moz-appearance: textfield;
  }
</style>
