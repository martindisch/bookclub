<script lang="ts">
  import Book from "../components/Book.svelte";
  import List from "../components/List.svelte";
  import Button from "../components/Button.svelte";

  const vote = () => alert("Voted for a book");
</script>

<List>
  <Book
    title="Hello, world!"
    author="Brian Kernighan"
    pageCount={200}
    description="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer molestie, augue quis sagittis egestas, nulla felis faucibus odio, vel vehicula metus nunc a justo. Proin elementum sapien sapien, fermentum ornare massa pellentesque non. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam accumsan risus non leo sollicitudin, sit amet pharetra augue cursus. Curabitur rutrum faucibus diam non volutpat. Mauris tincidunt a orci eu euismod. Morbi convallis finibus justo non porta. Sed nec efficitur risus, in volutpat lacus. Nam eleifend efficitur sapien in egestas. Aliquam cursus auctor mi eu semper. Maecenas consectetur nec velit nec efficitur. Ut dapibus metus ut congue tempor. Nunc faucibus, nisi egestas ornare euismod, velit tellus egestas magna, eu vehicula ante nunc non dui. Etiam congue purus felis, vitae mattis arcu tincidunt eget. Integer consequat fermentum nulla, in porta eros."
    pitchBy="Martin"
    firstSuggested={new Date("2021-12-05T16:53:25Z")}
    supporters={["Martin", "Alice", "Bob"]}
    onVote={vote}
  />
  <Book
    title="Goodbye and thanks for all the fish"
    author="Douglas Adams"
    pageCount={350}
    description="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer molestie, augue quis sagittis egestas, nulla felis faucibus odio, vel vehicula metus nunc a justo. Proin elementum sapien sapien, fermentum ornare massa pellentesque non. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam accumsan risus non leo sollicitudin, sit amet pharetra augue cursus."
    pitchBy="Alice"
    firstSuggested={new Date("2021-12-05T16:53:25Z")}
    supporters={["Alice"]}
    onVote={vote}
  />
  <Book
    title="A brief history of nothing"
    author="Paterson"
    pageCount={111}
    description="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer molestie, augue quis sagittis egestas, nulla felis faucibus odio, vel vehicula metus nunc a justo. Proin elementum sapien sapien, fermentum ornare massa pellentesque non. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam accumsan risus non leo sollicitudin, sit amet pharetra augue cursus. Curabitur rutrum faucibus diam non volutpat. Mauris tincidunt a orci eu euismod. Morbi convallis finibus justo non porta. Sed nec efficitur risus, in volutpat lacus. Nam eleifend efficitur sapien in egestas. Aliquam cursus auctor mi eu semper. Maecenas consectetur nec velit nec efficitur. Ut dapibus metus ut congue tempor. Nunc faucibus, nisi egestas ornare euismod, velit tellus egestas magna, eu vehicula ante nunc non dui. Etiam congue purus felis, vitae mattis arcu tincidunt eget. Integer consequat fermentum nulla, in porta eros."
    pitchBy="Bob"
    firstSuggested={new Date("2021-12-05T16:53:25Z")}
    supporters={["Bob", "Alice"]}
    onVote={vote}
  />
</List>
<Button text="+" rounded={true} />
