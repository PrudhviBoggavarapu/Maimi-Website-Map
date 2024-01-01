<script lang="ts">
	import { dataLoaded, responseData, selectedMuseum } from '$lib/shared/stores';
	import Dropdown from './Dropdown.svelte';
	let selectedMuseumLocal: any;
</script>

<header class="flex items-center w-full">
	<div class="flex-grow">
		<Dropdown bind:selectedMuseumLocal></Dropdown>
	</div>

	<label
		for="museum-select"
		class="bg-blue-500 text-white font-bold py-2 px-4 rounded w-32 h-10 flex items-center justify-center"
	>
		Count: {$responseData?.available.length}
	</label>

	<button
		on:click={() => {
			dataLoaded.set(false);
			//@ts-ignore
			selectedMuseum.set(selectedMuseumLocal);
		}}
		class="bg-blue-600 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded w-32 h-10 flex items-center justify-center"
	>
		{#if $dataLoaded}
			LOADED
		{:else}
			LOADING
		{/if}
	</button>
</header>
