<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";

  interface BookResponse {
        id: string,
        title: string,
        author: string,
        description: string,
        pageCount: number,
        pitchBy: string,
        firstSuggested: string,
        supporters: string[]
  }

  export const load: Load = async ({ page, fetch }) => {
    const res = await fetch(`${import.meta.env.VITE_API}/v1/books`);
    if (res.ok) {
      return {
        props: {
          books: await res.json(),
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

  export let books: BookResponse[];

  const vote = () => alert("Voted for a book");
</script>

<List>
{#each books as book (book.id)}
  <Book
    title={book.title}
    author={book.author}
    pageCount={book.pageCount}
    description={book.description}
    pitchBy={book.pitchBy}
    firstSuggested={new Date(Date.parse(book.firstSuggested))}
    supporters={book.supporters}
    onVote={vote}
  />
{/each}
</List>
<Button text="+" rounded={true} on:click={() => goto("/books/new")} />
