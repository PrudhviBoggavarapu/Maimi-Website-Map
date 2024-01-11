<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import DarkModeButton from '$lib/SvelteStuff/DarkModeButton.svelte';
	import Dropdown from './Dropdown.svelte';
	import { dataLoaded, responseData, selectedMuseum } from '$lib/shared/stores';
	import { Circle } from 'svelte-loading-spinners';
	let selectedMuseumLocal: any;
</script>

<div class="flex space-x-1.5 p-2 z-50">
	<DarkModeButton />
	<label
		for="Count of data"
		class="bg-primary text-primary-foreground py-2 px-4 rounded h-10 w-1/24 flex items-center text-sm"
		>Count:
		<div class="pl-3">
			{#if $responseData?.available.length == null}
				<Circle size="1" unit="rem" />
			{:else}
				{$responseData.available.length}
			{/if}
		</div>
	</label>
	<Dropdown bind:selectedMuseumLocal></Dropdown>
	<Button
		class="w-1/12 bg-primary text-primary-foreground flex items-center text-sm overflow-hidden content-around inline-box "
		on:click={() => {
			dataLoaded.set(false);
			selectedMuseum.set(selectedMuseumLocal);
			console.log(selectedMuseumLocal);
		}}
	>
		aaaaaaaa
		{#if $dataLoaded}
			LOADED
		{:else}
			LOADING
		{/if}</Button
	>
</div>
