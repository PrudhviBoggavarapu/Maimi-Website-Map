<script lang="ts">
	import { onMount } from 'svelte';
	import Map from '$lib/SvelteStuff/Map.svelte';
	import FetchStore from '$lib/SvelteStuff/FetchStore.svelte';
	import CleanData from '$lib/SvelteStuff/CleanData.svelte';
	import { isDarkReaderEnabled, urlBase64ToUint8Array, type Museum } from '$lib/shared/stores';
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	import Header from '$lib/SvelteStuff/Header.svelte';
	import { goto } from '$app/navigation';

	onMount(async () => {
		// Main JavaScript file (e.g., app.js)
		if ('serviceWorker' in navigator) {
			navigator.serviceWorker
				.register('/service-worker.js')
				.then(function (registration) {
					console.log('Service Worker registered with scope:', registration.scope);
					return registration.pushManager.getSubscription().then(function (subscription) {
						if (subscription) {
							return subscription;
						}

						const vapidPublicKey =
							'BB9FZK37PQyIOtQLVsxm_T7I_6dRz65xz_vCgODoJZKuscc3aJ8uo3koVFMgvP5d_v5IXliflKXCX6Mb9JUwqjo=';
						const convertedVapidKey = urlBase64ToUint8Array(vapidPublicKey);

						return registration.pushManager.subscribe({
							userVisibleOnly: true,
							applicationServerKey: convertedVapidKey
						});
					});
				})
				.then(function (subscription) {
					console.log(subscription.toJSON());
				})
				.catch(function (error) {
					console.error('Service Worker registration failed:', error);
				});
		}

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
