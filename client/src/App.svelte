<script>
	import axios from 'axios';

	import Card from './Card.svelte';
	import CardAdder from './CardAdder.svelte';

	let cards = [];

	async function getCards() {
		const response = await axios.get('http://localhost:8088/cards/query', {
			params: {
				user: 'odin',
				deck: 'test',
				page_number: 0,
				page_size: 5,
			}
		});

		cards = response.data.cards;
	}

	getCards();

	const cardAdded = () => getCards();
</script>

<h1>Flashcards App</h1>

<div>
	<CardAdder onAdd={cardAdded}/>
</div>

{#each cards as card}
	<Card word={card.item} definition={card.definition}/>
{/each}