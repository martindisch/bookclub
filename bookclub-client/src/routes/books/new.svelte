<script lang="ts">
  import BookForm, { Book } from "../../components/BookForm.svelte";
  import { responseToBook } from "../../api";
  import { goto } from "$app/navigation";

  const saveBook = async (book: Book) => {
    const res = await fetch(`${import.meta.env.VITE_API}/v1/books`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ...book,
        supporters: [book.pitchBy],
      }),
    });
    const newBook = responseToBook(await res.json());

    goto(`/#${newBook.id}`);
  };
</script>

<svelte:head>
  <title>New Book</title>
</svelte:head>

<BookForm onSave={saveBook} />
