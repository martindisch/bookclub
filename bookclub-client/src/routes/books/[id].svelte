<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";

  export const load: Load = async ({ page, fetch }) => {
    const res = await fetch(`${import.meta.env.VITE_API}/v1/books/${page.params.id}`);
    if (res.ok) {
      return {
        props: {
          book: await res.json(),
        },
      };
    }
    return {
      status: res.status,
      error: new Error("Could not load book"),
    };
  };
</script>

<script lang="ts">
  import BookForm, { Book } from "../../components/BookForm.svelte";
  import { goto } from "$app/navigation";
  import type { BookResponse } from "../../api";

  export let book: BookResponse;

  const updateBook = async (updatedBook: Book) => {
    await fetch(`${import.meta.env.VITE_API}/v1/books/${book.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(updatedBook),
    });

    goto("/");
  };
</script>

<BookForm {...book} onSave={updateBook} />
