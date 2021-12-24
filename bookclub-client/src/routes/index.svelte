<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";
  import { responseToBook, Book as BookProps } from "../api";

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
  import TextModal from "../components/TextModal.svelte";

  export let books: BookProps[];

  let showNamePrompt = false;

  const vote = async (id: string) => {
    const userName = localStorage.getItem("userName");
    if (userName === null) {
      showNamePrompt = true;
      return;
    }

    const res = await fetch(`${import.meta.env.VITE_API}/v1/books/${id}/supporters`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        supporter: userName,
      }),
    });

    const updatedBook = responseToBook(await res.json());
    // This is not great, ideally we'd pass the index into the current function
    const index = books.findIndex((b) => b.id === id);
    books[index] = updatedBook;
  };

  const storeNameAndClosePrompt = (userName: string) => {
    localStorage.setItem("userName", userName);
    showNamePrompt = false;
  };
</script>

{#if books.length > 0}
  <List>
    {#each books as book (book.id)}
      <Book {...book} onVote={() => vote(book.id)} />
    {/each}
  </List>
{:else}
  <p class="text-center">
    Looks like we don't have any books yet. Add some with the button in the bottom right corner.
  </p>
{/if}
<Button text="+" rounded={true} on:click={() => goto("/books/new")} />
{#if showNamePrompt}
  <TextModal prompt="Enter your name to start voting" onSave={storeNameAndClosePrompt} />
{/if}
