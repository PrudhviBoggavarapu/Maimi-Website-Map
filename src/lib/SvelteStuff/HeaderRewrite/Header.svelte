<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Dropdown from './Dropdown.svelte';
	import { dataLoaded, responseData, selectedMuseum } from '$lib/shared/stores';
	import { Circle } from 'svelte-loading-spinners';
	let selectedMuseumLocal: any;
</script>

<div class="flex space-x-1.5 p-2 z-50">
	<label
		for="Count of data"
		class="bg-blue-500 text-white py-2 px-4 rounded h-10 w-1/24 flex items-center text-sm"
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
		class="w-1/12"
		on:click={() => {
			dataLoaded.set(false);
			selectedMuseum.set(selectedMuseumLocal);
			console.log(selectedMuseumLocal);
		}}
	>
		{#if $dataLoaded}
			LOADED
		{:else}
			LOADING
		{/if}</Button
	>
</div>
