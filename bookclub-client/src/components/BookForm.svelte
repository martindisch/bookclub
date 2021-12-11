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
</script>

<Card>
  <form>
    <List>
      <Input label="Title" id="title" />
      <Input label="Author" id="author" />
      <Input label="Description" id="description" />
      <Input label="Page count" id="pageCount" />
      <Input label="Pitch by" id="pitchBy" />
    </List>
  </form>
</Card>
