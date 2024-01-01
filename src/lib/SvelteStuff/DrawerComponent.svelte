<!-- DrawerComponent.svelte -->
<script>
	import { Drawer, Button, CloseButton } from 'flowbite-svelte';
	import { InfoCircleSolid, ArrowRightOutline } from 'flowbite-svelte-icons';
	import { sineInOut } from 'svelte/easing';

	export let hidden = true; // Use export to allow the parent to control visibility

	// The transitionParams can be static if they don't need to be controlled by the parent.
	const transitionParams = {
		x: -320,
		duration: 150,
		easing: sineInOut
	};

	// Emit an event when the drawer is requested to close
	function requestClose() {
		hidden = true;
		dispatch('close');
	}
</script>

<div class="text-center bg-gray-800 py-4 deep-shadow z-10">
	<Button
		on:click={() => (hidden = false)}
		class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
	>
		Show drawer form
	</Button>
</div>

<Drawer
	transitionType="fly"
	{transitionParams}
	bind:hidden
	class="bg-inherit text-white w-1/4  z-10"
>
	<div class="flex items-center justify-between p-4 border-b border-gray-700 z-10">
		<h5 id="drawer-label" class="text-xl font-semibold">
			<InfoCircleSolid class="w-5 h-5 mr-2" />Info
		</h5>
		<CloseButton on:click={requestClose} class="text-gray-400 hover:text-gray-200 z-10" />
	</div>
	<div class="p-4 z-10">
		<div class="grid grid-cols-2 gap-4 z-10">
			<Button
				href="/"
				class="bg-transparent text-blue-500 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded z-10"
				>Learn more</Button
			>
		</div>
	</div>
</Drawer>

<style>
</style>
