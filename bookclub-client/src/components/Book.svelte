<script lang="ts">
  export let title: string;
  export let author: string;
  export let description: string;
  export let pageCount: number;
  export let pitchBy: string;
  export let firstSuggested: Date;
  export let supporters: string[];
</script>

<h1>{title}</h1>
<p>{author}</p>
<p>{description}</p>
<p>{pageCount}</p>
<p>{pitchBy}</p>
<p>{firstSuggested}</p>
<p>{supporters}</p>
