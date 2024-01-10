<script lang="ts">
	import {
		type PushNotificationConfig,
		registerServiceWorkerAndSubscribe,
		return_key_values
	} from '$lib/shared/stores';
	import { onMount } from 'svelte';
	import PocketBase from 'pocketbase';

	export async function addUserToApiUsers(userData: PushNotificationConfig) {
		try {
			const pb = new PocketBase('http://mapbackend.duckdns.org:8090/');
			await pb
				.collection('users')
				.authWithPassword(
					'NormalUserForPocketbase',
					'X@freHk*!84oyMdb6V93ubZf6bRHAoShBsrnRaRcgr#uf*#fNmutzciMRoJF!%JteJ5V@FLd'
				);
			const data = {
				Type: userData.type,
				Data: JSON.stringify(userData.data),
				museum: userData.museum
			};
			const record = await pb.collection('PushApi').create(data);
		} catch (error) {
			console.error('Error adding user:', error);
		}
	}
	onMount(async () => {
		try {
			let key_data = await registerServiceWorkerAndSubscribe();
			let cleaned_data = await return_key_values(key_data);

			if (cleaned_data) {
				await addUserToApiUsers(cleaned_data);
			} else {
				console.error('cleaned_data is undefined');
			}
		} catch (error) {
			console.error('Error in onMount:', error);
		}
	});
</script>
