<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { list_view_store } from '$lib/shared/stores';
	import { StretchHorizontalIcon } from 'lucide-svelte';
	export let list_view: boolean = false;
</script>

<Button
	on:click={() => {
		list_view = !list_view;
		list_view_store.set(list_view);
	}}
>
	<StretchHorizontalIcon />
</Button>
