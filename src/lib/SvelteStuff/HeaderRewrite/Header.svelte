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
	<Dropdown bind:selectedMuseumLocal></Dropdown>

	<label
		for="Count of data"
		class="bg-primary text-primary-foreground py-2 px-4 rounded h-10 w-fixed w-1/24 min-w-12 flex justify-center items-center text-center"
	>
		{#if $responseData?.available.length == null || !$dataLoaded}
			<div class="py-1">
				<Circle size=".9" unit="rem" />
			</div>
		{:else}
			{$responseData.available.length}
		{/if}
	</label>
</div>
