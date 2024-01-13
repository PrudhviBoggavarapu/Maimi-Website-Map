<script lang="ts">
	import type { Location } from '$lib/wasm-lib/pkg/wasm_lib';
	import * as Table from '$lib/components/ui/table';
	import { get } from 'svelte/store';
	import {
		type LocationWithDistance,
		cleanData,
		createGoogleMapsURL,
		createLibraryURL,
		selectedMuseum,
		sortByDistance,
		userLocation,
		createLocationWithDistance
	} from '$lib/shared/stores';

	let locations: LocationWithDistance[];

	$: {
		if ($cleanData) {
			let location = get(userLocation);
			if (location) {
				const sortedLocations = sortByDistance($cleanData, location[0], location[1]);
				locations = sortedLocations;
			} else {
				locations = $cleanData.map((location) => {
					// Since there's no reference point for distance, assign it to null
					return createLocationWithDistance(location, null);
				});
			}
		}
	}
</script>

<Table.Root>
	<Table.Caption>List of Locations that have the {$selectedMuseum.title}'s pass</Table.Caption>
	<Table.Header>
		<Table.Row>
			<Table.Head>Name</Table.Head>
			<Table.Head>Distance</Table.Head>
			<Table.Head>Address</Table.Head>
			<Table.Head>Phone</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each locations as locationWithDistance, i (i)}
			<Table.Row>
				<Table.Cell class="text-foreground">
					<a href={createLibraryURL(locationWithDistance.location)}>
						{locationWithDistance.location.name + ' Library'}
					</a>
				</Table.Cell>
				<Table.Cell class="text-foreground">
					{#if locationWithDistance.distance}
						{Math.round(locationWithDistance.distance * 100) / 100} miles
					{:else}
						'N/A'
					{/if}
				</Table.Cell>
				<Table.Cell class="text-foreground">
					<a href={createGoogleMapsURL(locationWithDistance.location.address)}>
						{locationWithDistance.location.address}
					</a>
				</Table.Cell>
				<Table.Cell class="text-foreground">
					<a href="tel:{locationWithDistance.location.phone.replaceAll('-', '')}">
						{locationWithDistance.location.phone}
					</a>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
