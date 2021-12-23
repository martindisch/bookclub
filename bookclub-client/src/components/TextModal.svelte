<script lang="ts">
  import Card from "./Card.svelte";
  import InputText from "./InputText.svelte";
  import Button from "./Button.svelte";

  export let prompt: string;
  export let value = "";
  export let onSave: (userName: string) => void;
</script>

<div class="fixed inset-0 bg-gray-600 bg-opacity-90" />
<div class="fixed inset-0 flex items-center justify-center w-full">
  <Card>
    <form on:submit|preventDefault={() => onSave(value)} class="flex flex-col gap-4">
      <InputText label={prompt} id="input" bind:value />
      <div class="place-self-end mt-2">
        <Button text="Save" submit={true} />
      </div>
    </form>
  </Card>
</div>
