<script lang="ts">
  import BookForm, { Book } from "../../components/BookForm.svelte";

  export let id: number;
  export let title: string;

  const saveBook = (book: Book) => {
    alert(JSON.stringify(book));
  };
</script>

<BookForm onSave={saveBook} />
