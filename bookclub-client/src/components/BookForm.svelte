<script lang="ts">
  import Card from "./Card.svelte";
  import Input from "./Input.svelte";
  import Button from "./Button.svelte";

  export let title: string | null = null;
  export let author: string | null = null;
  export let description: string | null = null;
  export let pageCount: number | null = null;
  export let pitchBy: string | null = null;

  const done = () => alert("Myello");
</script>

<Card>
  <form on:submit|preventDefault={done} class="grid gap-4 sm:grid-cols-2">
    <Input label="Title" id="title" />
    <Input label="Author" id="author" />
    <div class="col-span-full">
      <Input label="Description" id="description" rows={5} />
    </div>
    <Input label="Page count" id="pageCount" />
    <Input label="Pitch by" id="pitchBy" />
    <div class="col-span-full place-self-end mt-2">
      <Button text="Save" />
    </div>
  </form>
</Card>
