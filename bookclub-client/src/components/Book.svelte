<script lang="ts">
  import Card from "./Card.svelte";
  import Button from "./Button.svelte";

  export let title: string;
  export let author: string;
  export let description: string;
  export let pageCount: number;
  export let pitchBy: string;
  export let firstSuggested: Date;
  export let supporters: string[];
</script>

<Card>
  <h1 class="text-2xl">{title}</h1>
  <p class="mb-4">{author}</p>

  <p class="mb-4 hyphens-auto text-justify">{description}</p>

  <div class="grid grid-cols-3">
    <p>Page count</p>
    <p>{pageCount}</p>
    <div class="row-span-4 flex justify-end items-end">
      <Button text="Vote" />
    </div>
    <p>Pitch by</p>
    <p>{pitchBy}</p>
    <p>First suggested</p>
    <p>{firstSuggested.toLocaleDateString()}</p>
    <p>Supporters</p>
    <p>{supporters.join(", ")}</p>
  </div>
</Card>
