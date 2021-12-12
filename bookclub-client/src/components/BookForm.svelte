<script lang="ts" context="module">
  export interface Book {
    title: string;
    author: string;
    description: string;
    pageCount: number;
    pitchBy: string;
  }
</script>

<script lang="ts">
  import Card from "./Card.svelte";
  import InputText from "./InputText.svelte";
  import InputTextArea from "./InputTextArea.svelte";
  import InputNumber from "./InputNumber.svelte";
  import Button from "./Button.svelte";

  export let title = "";
  export let author = "";
  export let description = "";
  export let pageCount: number | null = null;
  export let pitchBy = "";
  export let onSave: (book: Book) => void;

  const save = () => {
    if (pageCount !== null) {
      onSave({ title, author, description, pageCount, pitchBy });
    }
  };
</script>

<Card>
  <form on:submit|preventDefault={save} class="grid gap-4 sm:grid-cols-2">
    <InputText label="Title" id="title" bind:value={title} />
    <InputText label="Author" id="author" bind:value={author} />
    <div class="col-span-full">
      <InputTextArea label="Description" id="description" rows={5} bind:value={description} />
    </div>
    <InputNumber label="Page count" id="pageCount" bind:value={pageCount} />
    <InputText label="Pitch by" id="pitchBy" bind:value={pitchBy} />
    <div class="col-span-full place-self-end mt-2">
      <Button text="Save" />
    </div>
  </form>
</Card>
