<script lang="ts">
	// Import the necessary variables and components
	import { museums, type Museum } from '$lib/shared/stores';

	export let selectedMuseumLocal: Museum | null = null;
	let selectedMuseumId = museums[0].id;

	// Function to handle dropdown selection
	function handleSelection(event: Event) {
		const target = event.target as HTMLSelectElement;
		selectedMuseumId = target.value;
		selectedMuseumLocal = museums.find((m) => m.id === selectedMuseumId) || null;
	}
</script>

<!-- Container for dropdown and button with flex layout -->
<div class="flex flex-row items-center">
	<select
		class="block appearance-none w-full border border-gray-900 hover:border-blue-400 bg-blue-400 px-4 py-2 pr-8 rounded leading-tight focus:outline-none text-center"
		bind:value={selectedMuseumId}
		on:change={handleSelection}
	>
		{#each museums as museum (museum.id)}
			<option value={museum.id}>{museum.title}</option>
		{/each}
	</select>
	<div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
		<svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
			<path d="M7.293 12.293a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414z" />
		</svg>
	</div>
</div>
