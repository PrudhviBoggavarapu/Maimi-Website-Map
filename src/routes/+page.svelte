<script lang="ts">
	import { onMount } from 'svelte';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import { isDarkReaderEnabled } from '$lib/shared/stores';
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	import Header from '$lib/SvelteStuff/HeaderRewrite/Header.svelte';
	import { goto } from '$app/navigation';
	import CleanData from '$lib/SvelteStuff/CleanData.svelte';
	import FetchStore from '$lib/SvelteStuff/FetchStore.svelte';

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
	});
</script>

<html lang="en"
	><div class="flex flex-col w-full h-screen">
		<Header />
		<Map></Map>
	</div>
	<FetchStore />
	<DetectDarkReader />
	<CleanData />
</html>

<style>
</style>
