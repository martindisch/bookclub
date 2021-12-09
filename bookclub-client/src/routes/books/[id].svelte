<script lang="ts" context="module">
  import type { Load } from "@sveltejs/kit";

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
</script>

<h1>{title}</h1>
<p>Here we go with ID {id}</p>
