<script lang="ts">
	import { onMount } from 'svelte';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import FetchStore from '$lib/SvelteStuff/FetchStore.svelte';
	import CleanData from '$lib/SvelteStuff/CleanData.svelte';
	import { isDarkReaderEnabled, type Museum } from '$lib/shared/stores';
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	import Header from '$lib/SvelteStuff/Header.svelte';
	import { goto } from '$app/navigation';

	onMount(async () => {
		isDarkReaderEnabled.subscribe((value) => {
			if (value) {
				document.body.classList.add('dark');
			} else {
				document.body.classList.remove('dark');
			}
		});

		if (localStorage.getItem('SawWelcome') !== 'true') {
			goto('/welcome');
		}

		let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');
		lib = await import('$lib/wasm-lib/pkg/wasm_lib');
		await lib.default();
		let oof = lib.get_best_match('allapattah');
	});
</script>

<html lang="en">
	<div class="flex flex-col w-full h-screen">
		<Header />

		<div class="flex-grow w-full">
			<Map></Map>
		</div>
	</div>

	<FetchStore />
	<CleanData />
	<DetectDarkReader />
</html>

<style>
</style>
