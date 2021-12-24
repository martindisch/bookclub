<script lang="ts">
  export let label: string;
  export let id: string;
  export let value = "";
</script>

<div class="flex flex-col gap-2">
  <label for={id}>{label}</label>
  <input
    autocapitalize="words"
    type="text"
    bind:value
    {id}
    class="bg-transparent rounded p-1 outline-solid-gray-700 focus:outline-solid-gray-800"
    required
  />
</div>
