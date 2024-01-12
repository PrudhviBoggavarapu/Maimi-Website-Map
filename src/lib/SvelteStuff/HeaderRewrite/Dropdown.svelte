<script lang="ts">
	import {
		Root as SelectRoot,
		Trigger as SelectTrigger,
		Value as SelectValue,
		Content as SelectContent,
		Item as SelectItem
	} from '$lib/components/ui/select';
	import { museums, type Museum, dataLoaded, selectedMuseum } from '$lib/shared/stores';

	let sorted_musium = museums.slice().sort((a, b) => a.title.localeCompare(b.title));

	export let selectedMuseumLocal: Museum | null = null;
	let selectedMuseumId = museums[0].id;

	// Function to handle dropdown selection
	function handleSelection(value: string) {
		selectedMuseumId = value;
		selectedMuseumLocal = museums.find((m) => m.id === selectedMuseumId) || null;
		dataLoaded.set(false);
		if (selectedMuseumLocal) {
			selectedMuseum.set(selectedMuseumLocal);
		} else {
			console.log('SOMETHING IS BROKEN IS DROPDOWN HANDLER FOR THE HEADER');
		}
	}
</script>

<SelectRoot typeahead={true} onSelectedChange={(event) => handleSelection(event.value)}>
	<SelectTrigger class="w-full text-foreground">
		<SelectValue placeholder="Select a Museum" />
	</SelectTrigger>
	<SelectContent>
		{#each sorted_musium as museum (museum.id)}
			<SelectItem value={museum.id}>{museum.title}</SelectItem>
		{/each}
	</SelectContent>
</SelectRoot>
