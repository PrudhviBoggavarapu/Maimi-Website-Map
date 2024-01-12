<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Button } from '$lib/components/ui/button';
	import { mediaQuery } from 'svelte-legos';
	import { Settings } from 'lucide-svelte';
	import SettingInternal from './SettingInternal.svelte';

	let open = false;
	const isDesktop = mediaQuery('(min-width: 768px)');
</script>

{#if $isDesktop}
	<Dialog.Root bind:open>
		<Dialog.Trigger asChild let:builder>
			<Button variant="outline" builders={[builder]}><Settings /></Button>
		</Dialog.Trigger>
		<Dialog.Content class="sm:max-w-[425px]">
			<Dialog.Header>
				<Dialog.Title>Settings</Dialog.Title>
				<Dialog.Description>Change settings for the app</Dialog.Description>
			</Dialog.Header>
			<SettingInternal />
		</Dialog.Content>
	</Dialog.Root>
{:else}
	<Drawer.Root bind:open>
		<Drawer.Trigger asChild let:builder>
			<Button variant="outline" builders={[builder]}><Settings /></Button>
		</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header class="text-left">
				<Drawer.Title>Settings</Drawer.Title>
				<Drawer.Description>Change settings for the app</Drawer.Description>
			</Drawer.Header>

			<div class="px-4"><SettingInternal /></div>

			<Drawer.Footer class="pt-2">
				<Drawer.Close asChild let:builder>
					<Button
						type="submit"
						builders={[builder]}
						on:click={() => {
							console.log('SUBMIT');
						}}>Save changes</Button
					>
					<Button variant="outline" builders={[builder]}>Cancel</Button>
				</Drawer.Close>
			</Drawer.Footer>
		</Drawer.Content>
	</Drawer.Root>
{/if}
