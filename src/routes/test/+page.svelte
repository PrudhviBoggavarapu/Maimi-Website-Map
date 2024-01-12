<script lang="ts">
	import SettingsButton from '$lib/SvelteStuff/HeaderRewrite/SettingButtonStuff/SettingsButton.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { isDarkReaderEnabled } from '$lib/shared/stores';
	import { onMount } from 'svelte';

	onMount(() => {
		const currentColor = localStorage.getItem('SawWelcome');
		console.log(currentColor);
	});
</script>

<!-- The UI part of your component -->
<div class="flex-col p-2">
	<Label>
		Dark Reader is {$isDarkReaderEnabled ? 'enabled' : 'disabled'}.
	</Label>
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

	<SettingsButton />
</div>
