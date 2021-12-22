<script lang="ts">
  import BookForm, { Book } from "../../components/BookForm.svelte";

  const saveBook = (book: Book) => {
    alert(JSON.stringify(book));
  };
</script>

<BookForm onSave={saveBook} />
