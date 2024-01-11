<script lang="ts">
	import { onMount } from 'svelte';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import { dark_mode_handler } from '$lib/shared/stores';
	import Header from '$lib/SvelteStuff/HeaderRewrite/Header.svelte';
	import { goto } from '$app/navigation';
	import { prelude_data } from '$lib/preload';

	onMount(async () => {
		await prelude_data();
		dark_mode_handler.subscribe((value) => {
			console.log(value);
		});

		if (localStorage.getItem('SawWelcome') !== 'true') {
			goto('/welcome');
		}
	});
</script>

<body lang="en" class="bg-background"
	><div class="flex flex-col w-full h-screen">
		<Header />
		<Map></Map>
	</div>
</body>

<style>
</style>
