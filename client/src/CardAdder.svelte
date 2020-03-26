<script>
    import axios from 'axios';

    let item = '';
    let definition = '';

    export let onAdd;

    async function addCard() {
        if (!item || !definition) {
            return;
        }
        
        console.log(`Adding card: ${item}, ${definition}`);

        await axios.post('http://localhost:8088/cards', {
            user: 'odin',
            deck: 'test',
            item: item,
            definition: definition,
        });

        onAdd();
    }
</script>

<style>
    input {
        width: 30%;
        margin: 10px;
    }
</style>

<div>
    <h2>Add Card</h2>

    <div>
        <input bind:value={item} placeholder="item">
        <input bind:value={definition} placeholder="definition">
    </div>

    <button class="waves-effect waves-light btn" on:click={addCard}>
        Add Card
    </button>
</div>