<script>
    export let word;
    export let definition;

    let showDefinition = false;

    let toggleDefinition = () => showDefinition = !showDefinition; 
</script>

<h2>{word}</h2>
<p>{word}</p>

<button on:click={toggleDefinition}>Show Definition</button>

{#if showDefinition}
    <h3>{definition}</h3>
{/if}