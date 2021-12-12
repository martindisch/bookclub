<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";
  import BookForm, { Book } from "../../components/BookForm.svelte";

  export const load: Load = async ({ page, fetch }) => {
    const res = await fetch("https://jsonplaceholder.typicode.com/todos/1");

    if (res.ok) {
      return {
        props: {
          title: (await res.json()).title,
          id: page.params.id,
        },
      };
    }

    return {
      status: res.status,
      error: new Error("Could not load user"),
    };
  };
</script>

<script lang="ts">
  export let id: number;
  export let title: string;

  const saveBook = (book: Book) => {
    alert(JSON.stringify(book));
  };
</script>

<BookForm onSave={saveBook} />
