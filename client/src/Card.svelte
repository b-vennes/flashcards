<script>
    export let word;
    export let definition;

    let showDefinition = false;

    let toggleDefinition = () => showDefinition = !showDefinition;
</script>

<div>
    <h3>{word}</h3>
    <p>{word}</p>

    <button class="waves-effect waves-light btn" on:click={toggleDefinition}>Show Definition</button>

    {#if showDefinition}
        <p>{definition}<p/>
    {/if}
</div>