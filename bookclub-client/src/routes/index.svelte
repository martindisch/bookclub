<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";
  import { responseToBook } from "../api";
  import type { Book as BookProps } from "../types";

  export const load: Load = async ({ fetch }) => {
    const res = await fetch(`${import.meta.env.VITE_API}/v1/books`);
    if (res.ok) {
      return {
        props: {
          books: (await res.json()).map(responseToBook),
        },
      };
    }
    return {
      status: res.status,
      error: new Error("Could not load books"),
    };
  };
</script>

<script lang="ts">
  import { goto } from "$app/navigation";
  import Book from "../components/Book.svelte";
  import List from "../components/List.svelte";
  import Button from "../components/Button.svelte";

  export let books: BookProps[];

  const vote = () => alert("Voted for a book");
</script>

<List>
  {#each books as book (book.id)}
    <Book {...book} onVote={vote} href={`/books/${book.id}`} />
  {/each}
</List>
<Button text="+" rounded={true} on:click={() => goto("/books/new")} />
