<script lang="ts">
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	import { isDarkReaderEnabled } from '$lib/shared/stores';
	import { onMount } from 'svelte';

	import { Circle3 } from 'svelte-loading-spinners';

	onMount(() => {
		const currentColor = localStorage.getItem('SawWelcome');
		console.log(currentColor);
	});
</script>

<!-- The UI part of your component -->
<div>
	Dark Reader is {$isDarkReaderEnabled ? 'enabled' : 'disabled'}.
	<DetectDarkReader />
	<button
		class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded transition duration-300 ease-in-out transform hover:-translate-y-1 hover:shadow-lg"
		on:click={() => {
			localStorage.removeItem('SawWelcome');
		}}
		>CLEAR localStorage
	</button>

	<button
		class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded transition duration-300 ease-in-out transform hover:-translate-y-1 hover:shadow-lg"
		on:click={() => {
			if ('serviceWorker' in navigator) {
				navigator.serviceWorker.getRegistrations().then(function (registrations) {
					for (const registration of registrations) {
						registration
							.unregister()
							.then(function () {
								console.log('Cleared the worker');
							})
							.catch(function (error) {
								console.log('Could not cleared the worker');
							});
					}
				});
			}
		}}
	>
		Clear Worker	
	</button>
</div>
