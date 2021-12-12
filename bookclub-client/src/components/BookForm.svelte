<script lang="ts">
  import Card from "./Card.svelte";
  import InputText from "./InputText.svelte";
  import InputTextArea from "./InputTextArea.svelte";
  import InputNumber from "./InputNumber.svelte";
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
    <InputText label="Title" id="title" />
    <InputText label="Author" id="author" />
    <div class="col-span-full">
      <InputTextArea label="Description" id="description" rows={5} />
    </div>
    <InputNumber label="Page count" id="pageCount" />
    <InputText label="Pitch by" id="pitchBy" />
    <div class="col-span-full place-self-end mt-2">
      <Button text="Save" />
    </div>
  </form>
</Card>
