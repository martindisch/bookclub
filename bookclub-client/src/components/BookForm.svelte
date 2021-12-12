<script lang="ts">
  import Card from "./Card.svelte";
  import Input from "./Input.svelte";
  import List from "./List.svelte";
  import Button from "./Button.svelte";

  export let title: string | null = null;
  export let author: string | null = null;
  export let description: string | null = null;
  export let pageCount: number | null = null;
  export let pitchBy: string | null = null;

  const done = () => alert("Myello");
</script>

<Card>
  <form on:submit|preventDefault={done}>
    <List>
      <Input label="Title" id="title" />
      <Input label="Author" id="author" />
      <Input label="Description" id="description" rows={5} />
      <Input label="Page count" id="pageCount" />
      <Input label="Pitch by" id="pitchBy" />
      <div class="place-self-end mt-2"><Button text="Save" /></div>
    </List>
  </form>
</Card>
