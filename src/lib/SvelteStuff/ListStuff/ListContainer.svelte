<script lang="ts">
	import type { Location } from '$lib/wasm-lib/pkg/wasm_lib';
	import * as Table from '$lib/components/ui/table';
	import { cleanData, createGoogleMapsURL, selectedMuseum } from '$lib/shared/stores';
	let locations: Location[];

	$: {
		if ($cleanData) {
			locations = $cleanData;
		}
		console.log(locations);
	}
</script>

<Table.Root>
	<Table.Caption>List of Locations that have the {$selectedMuseum.title}'s pass</Table.Caption>
	<Table.Header>
		<Table.Row>
			<Table.Head>Name</Table.Head>
			<Table.Head>Address</Table.Head>
			<Table.Head>Phone</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each locations as location, i (i)}
			<Table.Row>
				<Table.Cell class="text-foreground">{location.name + ' Library'}</Table.Cell>
				<Table.Cell class="text-foreground"
					><a href={createGoogleMapsURL(location.address)}>{location.address}</a></Table.Cell
				>
				<Table.Cell class="text-foreground"
					><a href="tel:{location.phone.replaceAll('-', '')}">{location.phone}</a></Table.Cell
				>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
