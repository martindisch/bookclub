<script lang="ts">
  export let title: string;
  export let author: string;
  export let description: string;
  export let pageCount: number;
  export let pitchBy: string;
  export let firstSuggested: Date;
  export let supporters: string[];
</script>

<div class="bg-gray-500 rounded-xl p-6">
  <h1 class="text-2xl">{title}</h1>
  <p class="mb-4">{author}</p>

  <p class="mb-4 hyphens-auto text-justify">{description}</p>

  <div class="grid grid-cols-3">
    <p>Page count</p>
    <p>{pageCount}</p>
    <div class="row-span-4 flex justify-end items-end">
      <button
        class="bg-green-700 hover:bg-green-800 active:bg-green-700 py-2 px-8 rounded-md shadow focus:outline-none"
        >Vote</button
      >
    </div>
    <p>Pitch by</p>
    <p>{pitchBy}</p>
    <p>First suggested</p>
    <p>{firstSuggested.toLocaleDateString()}</p>
    <p>Supporters</p>
    <p>{supporters.join(", ")}</p>
  </div>
</div>
