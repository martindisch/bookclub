<script lang="ts">
  export let text: string;
  export let rounded = false;
  export let submit = false;
</script>

<button
  class="bg-green-700 hover:bg-green-800 active:bg-green-700 shadow focus:outline-none {rounded
    ? 'rounded-full w-16 h-16 fixed right-4 bottom-4 text-4xl'
    : 'rounded-md py-2 px-8'}"
  on:click
  type={submit ? "submit" : "button"}
>
  {text}
</button>
