<script lang="ts">
	import { cleanData, get_api_and_store, responseData, selectedMuseum } from '../shared/stores';
	import { onMount } from 'svelte';
	let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');

	onMount(async () => {
		lib = await import('$lib/wasm-lib/pkg/wasm_lib');
		await lib.default();

		await selectedMuseum.subscribe((storeValue) => {
			console.log('GET URL', storeValue);
			if (storeValue) {
				const url = `https://na.iiivega.com/api/search-result/drawer/format-groups/${storeValue.id}/locations?tab=Museum%20Pass`;
				let x = get_api_and_store(url);
				x.then((item) => {});
			}
		});
	});
</script>
