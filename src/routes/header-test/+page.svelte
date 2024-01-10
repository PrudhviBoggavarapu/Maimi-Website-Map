<script lang="ts">
	import * as Select from '$lib/components/ui/select';

	// Define your fruits array as before
	const fruits = [{ value: 'apple', label: 'Apple' }];

	// Define a variable to hold the selected value
	let selectedFruit = '';

	// Function to handle selection change (optional)
	function handleSelectionChange() {
		console.log('Selected fruit: ', selectedFruit);
	}
</script>

<Select.Root
	portal={null}
	
	onSelectedChange={(event) => {
		console.log(event);
	}}
>
	<Select.Trigger class="w-[180px]">
		<Select.Value placeholder="Select a fruit" />
	</Select.Trigger>
	<Select.Content>
		<Select.Group>
			<Select.Label>Fruits</Select.Label>
			{#each fruits as fruit}
				<Select.Item value={fruit.value} label={fruit.label}>{fruit.label}</Select.Item>
			{/each}
		</Select.Group>
	</Select.Content>
	<Select.Input name="favoriteFruit" />
</Select.Root>
