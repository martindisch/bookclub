<script>
  import Book from "../components/Book.svelte";
</script>

<Book
  title="Hello, world!"
  author="Brian Kernighan"
  pageCount={200}
  description="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer molestie, augue quis sagittis egestas, nulla felis faucibus odio, vel vehicula metus nunc a justo. Proin elementum sapien sapien, fermentum ornare massa pellentesque non. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam accumsan risus non leo sollicitudin, sit amet pharetra augue cursus. Curabitur rutrum faucibus diam non volutpat. Mauris tincidunt a orci eu euismod. Morbi convallis finibus justo non porta. Sed nec efficitur risus, in volutpat lacus. Nam eleifend efficitur sapien in egestas. Aliquam cursus auctor mi eu semper. Maecenas consectetur nec velit nec efficitur. Ut dapibus metus ut congue tempor. Nunc faucibus, nisi egestas ornare euismod, velit tellus egestas magna, eu vehicula ante nunc non dui. Etiam congue purus felis, vitae mattis arcu tincidunt eget. Integer consequat fermentum nulla, in porta eros."
  pitchBy="Martin"
  firstSuggested={new Date("2021-12-05T16:53:25Z")}
  supporters={["Martin", "Alice", "Bob"]}
/>
