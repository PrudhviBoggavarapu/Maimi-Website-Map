<script lang="ts">
	import { Sun, Moon } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { isDarkReaderEnabled, targetDarkMode } from '$lib/shared/stores';
	import { get } from 'svelte/store';
	import { onMount } from 'svelte';
	let isDark: boolean = false;
	onMount(() => {
		isDark = get(isDarkReaderEnabled);
	});

	function toggleTheme() {
		isDark = !isDark;
	}
</script>

<Button
	on:click={() => {
		toggleTheme();
		targetDarkMode.set(isDark);
	}}
>
	{#if isDark}
		<Moon />
	{:else}
		<Sun />
	{/if}
</Button>
