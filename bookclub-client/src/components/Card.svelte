<script lang="ts">
  export let id: string | null = null;
</script>

<div class="bg-gray-500 rounded-xl p-6" {id}><slot /></div>
