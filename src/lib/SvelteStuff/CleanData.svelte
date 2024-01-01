<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');
	import { responseData, type StoreData, cleanData } from '../shared/stores';

	onMount(async () => {
		// import Wasm
		lib = await import('$lib/wasm-lib/pkg/wasm_lib');
		await lib.default();

		responseData.subscribe((value) => {
			let currentData = value;
			if (currentData !== null) {
				const cleaned_data = currentData.available.map((item) => {
					return lib.get_best_match(item.itemLocationLabel);
				});
				cleanData.set(cleaned_data);
			}
		});
	});
	onDestroy(() => {});
</script>
