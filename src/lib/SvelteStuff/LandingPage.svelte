<script lang="ts">
	import { onMount } from 'svelte';
	import branch_json from '$lib/json/branches_to_adresses.json?raw';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import FetchStore from '$lib/SvelteStuff/FetchStore.svelte';
	import CleanData from '$lib/SvelteStuff/CleanData.svelte';
	let a: string = 'Loading Query';
	onMount(async () => {
		let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');
		lib = await import('$lib/wasm-lib/pkg/wasm_lib');
		await lib.default();
		lib.greet();
		let oof = lib.get_best_match('allapattah', branch_json);
		a = oof.address;
	});
</script>

<html lang="en">
	<Map />
	<FetchStore />
	<CleanData />
</html>

<style>
</style>
