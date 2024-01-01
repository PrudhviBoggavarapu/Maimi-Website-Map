<script lang="ts">
	import { onMount } from 'svelte';
	import { Drawer, Button, CloseButton } from 'flowbite-svelte';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import FetchStore from '$lib/SvelteStuff/FetchStore.svelte';
	import CleanData from '$lib/SvelteStuff/CleanData.svelte';
	import DrawerComponent from '$lib/SvelteStuff/DrawerComponent.svelte';
	import {
		dataLoaded,
		isDarkReaderEnabled,
		museums,
		selectedMuseum,
		type Museum,
		responseData
	} from '$lib/shared/stores';
	import Dropdown from '$lib/SvelteStuff/Dropdown.svelte';
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	let a: string = 'Loading Query';

	let isDrawerHidden = true;

	onMount(async () => {
		isDarkReaderEnabled.subscribe((value) => {
			if (value) {
				document.body.classList.add('dark');
			} else {
				document.body.classList.remove('dark');
			}
		});

		let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');
		lib = await import('$lib/wasm-lib/pkg/wasm_lib');
		await lib.default();
		let oof = lib.get_best_match('allapattah');
		a = oof.address;
	});
	let selectedMuseumLocal: Museum | null | undefined;
</script>

<html lang="en">
	<div class="flex items-center w-full">
		<div class="flex-grow">
			<Dropdown bind:selectedMuseumLocal></Dropdown>
		</div>

		<label
			for="museum-select"
			class="text-gray-700 font-bold bg-white border border-gray-300 hover:border-gray-400 ml-4"
		>
			Count: {$responseData?.available.length}
		</label>
		{#if $dataLoaded}
			<button
				on:click={() => {
					dataLoaded.set(false);
					//@ts-ignore
					selectedMuseum.set(selectedMuseumLocal);
				}}
				for="museum-select"
				class="text-gray-700 font-bold bg-white border border-gray-300 hover:border-gray-400 ml-4"
			>
				LOADED
			</button>
		{:else}
			<button
				on:click={() => {
					dataLoaded.set(false);
					//@ts-ignore
					selectedMuseum.set(selectedMuseumLocal);
				}}
				for="museum-select"
				class="text-gray-700 font-bold bg-white border border-gray-300 hover:border-gray-400 ml-4"
			>
				LOADING
			</button>
		{/if}
		<!-- <button
			on:click={() => {
				dataLoaded.set(false);
				//@ts-ignore
				selectedMuseum.set(selectedMuseumLocal);
			}}
			class="rounded-lg bg-black text-white ml-4"
		>
			Click Here To Map
		</button> -->
	</div>

	<Map></Map>

	<FetchStore />
	<CleanData />
	<DetectDarkReader />
</html>

<style>
</style>
